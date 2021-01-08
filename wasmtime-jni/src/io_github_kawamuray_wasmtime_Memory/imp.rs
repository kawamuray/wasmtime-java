use super::JniMemory;
use crate::errors::{self, Result};
use crate::interop;
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong, jobject};
use jni::JNIEnv;
use wasmtime::{Limits, Memory, MemoryType, Store};

pub(super) struct JniMemoryImpl;

impl<'a> JniMemory<'a> for JniMemoryImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::take_inner::<Memory>(&env, this)?;
        Ok(())
    }

    fn buffer(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error> {
        let mem = interop::get_inner::<Memory>(&env, this)?;
        let ptr = unsafe { mem.data_unchecked_mut() };
        Ok(env.new_direct_byte_buffer(ptr)?.into_inner())
    }

    fn data_size(env: &JNIEnv, this: JObject) -> Result<jlong, Self::Error> {
        let mem = interop::get_inner::<Memory>(&env, this)?;
        Ok(mem.data_size() as jlong)
    }

    fn new_memory(
        _env: &JNIEnv,
        _clazz: JClass,
        store_ptr: jlong,
        min: jint,
        max: jint,
    ) -> Result<jlong, Self::Error> {
        let store = interop::ref_from_raw::<Store>(store_ptr)?;
        let ty = MemoryType::new(Limits::new(
            min as u32,
            if max < 0 { None } else { Some(max as u32) },
        ));
        let mem = Memory::new(&store, ty);
        Ok(interop::into_raw::<Memory>(mem))
    }

    fn size(env: &JNIEnv, this: JObject) -> Result<jint, Self::Error> {
        let mem = interop::get_inner::<Memory>(&env, this)?;
        Ok(mem.size() as jint)
    }

    fn grow(env: &JNIEnv, this: JObject, page: jint) -> Result<jint, Self::Error> {
        let mem = interop::get_inner::<Memory>(&env, this)?;
        let re = mem.grow(page as u32)? as i32;
        Ok(re)
    }
}
