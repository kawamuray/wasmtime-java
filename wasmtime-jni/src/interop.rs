use crate::errors::Result;
use jni::descriptors::Desc;
use jni::errors::Result as JniResult;
use jni::objects::{JFieldID, JObject};
use jni::signature::{JavaType, Primitive};
use jni::strings::JNIString;
use jni::sys::jlong;
use jni::JNIEnv;
use std::sync::Mutex;
use std::sync::MutexGuard;

pub const INNER_PTR_FIELD: &str = "innerPtr";

/// Surrender a Rust object into a pointer.
/// The given value gets "forgotten" by Rust's memory management
/// so you have to get it back into a `T` at some point to avoid leaking memory.
pub fn into_raw<T>(val: T) -> jlong
where
    T: 'static,
{
    Box::into_raw(Box::new(Mutex::new(val))) as jlong
}

/// Restore a Rust object of type `T` from a pointer.
/// This is the reverse operation of `into_raw`.
pub fn from_raw<T>(ptr: jlong) -> Result<T> {
    Ok((*unsafe { Box::from_raw(ptr as *mut Mutex<T>) }).into_inner()?)
}

pub fn ref_from_raw<'a, T>(ptr: jlong) -> Result<MutexGuard<'a, T>> {
    let ptr = ptr as *mut Mutex<T>;
    Ok(unsafe { (*ptr).lock()? })
}

macro_rules! non_null {
    ( $obj:expr, $ctx:expr ) => {
        if $obj.is_null() {
            return Err(jni::errors::ErrorKind::NullPtr($ctx).into());
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
        .j()? as *mut Mutex<T>;
    if !field_ptr.is_null() {
        return Err(format!("field already set: {}", field.as_ref()).into());
    }

    let ptr = into_raw(rust_object);

    env.set_field_unchecked(obj, field_id, (ptr as jni::sys::jlong).into())
}

/// A port of `JNIEnv::get_rust_field` with type `T` modified to not require `Send`.
pub fn get_field<'a, O, S, T>(env: &JNIEnv<'a>, obj: O, field: S) -> JniResult<MutexGuard<'a, T>>
where
    O: Into<JObject<'a>>,
    S: Into<JNIString>,
    T: 'static,
{
    let obj = obj.into();
    let _guard = env.lock_obj(obj)?;

    let ptr = env.get_field(obj, field, "J")?.j()? as *mut Mutex<T>;
    non_null!(ptr, "rust value from Java");
    unsafe {
        // dereferencing is safe, because we checked it for null
        Ok((*ptr).lock().unwrap())
    }
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

    let mbox = {
        let _guard = env.lock_obj(obj)?;

        let ptr = env
            .get_field_unchecked(obj, field_id, JavaType::Primitive(Primitive::Long))?
            .j()? as *mut Mutex<T>;

        non_null!(ptr, "rust value from Java");

        let mbox = unsafe { Box::from_raw(ptr) };

        // attempt to acquire the lock. This prevents us from consuming the
        // mutex if there's an outstanding lock. No one else will be able to
        // get a new one as long as we're in the guarded scope.
        drop(mbox.try_lock()?);

        env.set_field_unchecked(
            obj,
            field_id,
            (::std::ptr::null_mut::<()>() as jni::sys::jlong).into(),
        )?;

        mbox
    };

    Ok(mbox.into_inner().unwrap())
}

pub fn set_inner<'a, O, S, T>(env: &JNIEnv<'a>, obj: JObject<'a>, rust_object: T) -> JniResult<()>
where
    T: 'static,
{
    set_field(env, obj, INNER_PTR_FIELD, rust_object)
}

pub fn get_inner<'a, T>(env: &JNIEnv<'a>, obj: JObject<'a>) -> JniResult<MutexGuard<'a, T>>
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
