use crate::errors::Result;
use jni::descriptors::Desc;
use jni::errors::{Error, Result as JniResult};
use jni::objects::{JFieldID, JObject};
use jni::signature::{JavaType, Primitive};
use jni::strings::JNIString;
use jni::sys::jlong;
use jni::JNIEnv;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

pub const INNER_PTR_FIELD: &str = "innerPtr";

pub struct ReentrantLock<T> {
    mutex: Mutex<T>,
    current_owner: AtomicU64,
}

pub enum ReentrantReference<'a, T> {
    Locked(&'a ReentrantLock<T>, std::sync::MutexGuard<'a, T>),
    Recursive(&'a mut T),
}

impl<'a, T> Drop for ReentrantReference<'a, T> {
    fn drop(&mut self) {
        match self {
            ReentrantReference::Locked(lock, _) => {
                lock.current_owner.store(0, Ordering::Relaxed);
            }
            ReentrantReference::Recursive(_) => {}
        }
    }
}

impl<T> ReentrantLock<T> {
    pub fn new(val: T) -> Self {
        Self {
            mutex: Mutex::new(val),
            current_owner: AtomicU64::new(0),
        }
    }

    fn lock(&mut self) -> Result<ReentrantReference<'_, T>> {
        let current_id = std::thread::current().id().as_u64().get();
        if current_id == self.current_owner.load(Ordering::Relaxed) {
            let reference = self.mutex.get_mut()?;
            return Ok(ReentrantReference::Recursive(reference));
        }
        let guard = self.mutex.lock()?;
        self.current_owner.store(current_id, Ordering::Relaxed);
        Ok(ReentrantReference::Locked(self, guard))
    }
}

impl<'a, T> std::ops::Deref for ReentrantReference<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ReentrantReference::Locked(_, lock) => &*lock,
            ReentrantReference::Recursive(r) => r,
        }
    }
}

impl<'a, T> std::ops::DerefMut for ReentrantReference<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ReentrantReference::Locked(_, lock) => &mut *lock,
            ReentrantReference::Recursive(r) => r,
        }
    }
}

/// Surrender a Rust object into a pointer.
/// The given value gets "forgotten" by Rust's memory management
/// so you have to get it back into a `T` at some point to avoid leaking memory.
pub fn into_raw<T>(val: T) -> jlong
where
    T: 'static,
{
    Box::into_raw(Box::new(ReentrantLock::new(val))) as jlong
}

/// Restore a Rust object of type `T` from a pointer.
/// This is the reverse operation of `into_raw`.
pub fn from_raw<T>(ptr: jlong) -> Result<T> {
    Ok((*unsafe { Box::from_raw(ptr as *mut Mutex<T>) }).into_inner()?)
}

pub fn ref_from_raw<'a, T>(ptr: jlong) -> Result<ReentrantReference<'a, T>> {
    let ptr = ptr as *mut ReentrantLock<T>;
    unsafe { (*ptr).lock() }
}

macro_rules! non_null {
    ( $obj:expr, $ctx:expr ) => {
        if $obj.is_null() {
            return Err(jni::errors::Error::NullPtr($ctx).into());
        } else {
            $obj
        }
    };
}

/// A port of `JNIEnv::set_rust_field` with type `T` modified to not require `Send`.
/// It still preserves Mutex around the value, not for atomic access but for making sure
/// the unique owner at the time it is taken.
pub fn set_field<'a, O, S, T>(env: &JNIEnv<'a>, obj: O, field: S, rust_object: T) -> JniResult<()>
where
    O: Into<JObject<'a>>,
    S: AsRef<str>,
    T: 'static,
{
    let obj = obj.into();
    let class = env.auto_local(env.get_object_class(obj)?);
    let field_id: JFieldID = (&class, &field, "J").lookup(env)?;

    let _guard = env.lock_obj(obj)?;

    // Check to see if we've already set this value. If it's not null, that
    // means that we're going to leak memory if it gets overwritten.
    let field_ptr = env
        .get_field_unchecked(obj, field_id, JavaType::Primitive(Primitive::Long))?
        .j()? as *mut ReentrantLock<T>;
    if !field_ptr.is_null() {
        return Err(Error::FieldAlreadySet(field.as_ref().to_string()));
    }

    let ptr = into_raw(rust_object);

    env.set_field_unchecked(obj, field_id, (ptr as jni::sys::jlong).into())
}

/// A port of `JNIEnv::get_rust_field` with type `T` modified to not require `Send`.
pub fn get_field<'a, O, S, T>(
    env: &JNIEnv<'a>,
    obj: O,
    field: S,
) -> JniResult<ReentrantReference<'a, T>>
where
    O: Into<JObject<'a>>,
    S: Into<JNIString>,
    T: 'static,
{
    let obj = obj.into();
    let _guard = env.lock_obj(obj)?;

    let ptr = env.get_field(obj, field, "J")?.j()? as *mut ReentrantLock<T>;
    non_null!(ptr, "rust value from Java");
    unsafe {
        // dereferencing is safe, because we checked it for null
        Ok((*ptr).lock().unwrap())
    }
}

fn inner_ptr<'a>(env: &JNIEnv<'a>, obj: JObject<'a>) -> JniResult<jlong> {
    let class = env.auto_local(env.get_object_class(obj)?);
    let field_id: JFieldID = (&class, INNER_PTR_FIELD, "J").lookup(env)?;
    Ok(env
        .get_field_unchecked(obj, field_id, JavaType::Primitive(Primitive::Long))?
        .j()?)
}

/// A port of `JNIEnv::take_rust_field` with type `T` modified to not require `Send`.
pub fn take_field<'a, O, S, T>(env: &JNIEnv<'a>, obj: O, field: S) -> JniResult<T>
where
    O: Into<JObject<'a>>,
    S: AsRef<str>,
    T: 'static,
{
    let obj = obj.into();
    let class = env.auto_local(env.get_object_class(obj)?);
    let field_id: JFieldID = (&class, &field, "J").lookup(env)?;

    let _guard = env.lock_obj(obj)?;
    let mbox = {
        let ptr = env
            .get_field_unchecked(obj, field_id, JavaType::Primitive(Primitive::Long))?
            .j()? as *mut ReentrantLock<T>;

        non_null!(ptr, "rust value from Java");

        let mbox = unsafe { Box::from_raw(ptr) };

        // attempt to acquire the lock. This prevents us from consuming the
        // mutex if there's an outstanding lock. No one else will be able to
        // get a new one as long as we're in the guarded scope.
        drop(mbox.mutex.lock());

        env.set_field_unchecked(
            obj,
            field_id,
            (::std::ptr::null_mut::<()>() as jni::sys::jlong).into(),
        )?;

        mbox
    };

    Ok(mbox.mutex.into_inner().unwrap())
}

pub fn set_inner<'a, O, S, T>(env: &JNIEnv<'a>, obj: JObject<'a>, rust_object: T) -> JniResult<()>
where
    T: 'static,
{
    set_field(env, obj, INNER_PTR_FIELD, rust_object)
}

pub fn get_inner<'a, T>(env: &JNIEnv<'a>, obj: JObject<'a>) -> JniResult<ReentrantReference<'a, T>>
where
    T: 'static,
{
    get_field(env, obj, INNER_PTR_FIELD)
}

pub fn take_inner<'a, T>(env: &JNIEnv<'a>, obj: JObject<'a>) -> JniResult<T>
where
    T: 'static,
{
    take_field(env, obj, INNER_PTR_FIELD)
}

pub fn dispose_inner<'a, T>(env: &JNIEnv<'a>, obj: JObject<'a>) -> JniResult<()>
where
    T: 'static,
{
    if inner_ptr(env, obj)? == 0 {
        return Ok(());
    }
    take_field::<_, _, T>(env, obj, INNER_PTR_FIELD)?;
    Ok(())
}
