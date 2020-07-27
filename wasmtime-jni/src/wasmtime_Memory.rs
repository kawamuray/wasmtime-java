use crate::errors::{self, Result};
use crate::{interop, wrap_error};
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong, jobject};
use jni::JNIEnv;
use wasmtime::{Limits, Memory, MemoryType, Store};

fn new_memory(store_ptr: jlong, limit_min: jint, limit_max: jint) -> Result<jlong> {
    let store = interop::ref_from_raw::<Store>(store_ptr)?;
    let ty = MemoryType::new(Limits::new(
        limit_min as u32,
        if limit_max < 0 {
            None
        } else {
            Some(limit_max as u32)
        },
    ));
    let mem = Memory::new(&store, ty);
    Ok(interop::into_raw::<Memory>(mem))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Memory_newMemory(
    env: JNIEnv,
    _clazz: JClass,
    store_ptr: jlong,
    limit_min: jint,
    limit_max: jint,
) -> jlong {
    wrap_error!(env, new_memory(store_ptr, limit_min, limit_max))
}

fn mem_data_size(env: &JNIEnv, this: JObject) -> Result<jlong> {
    let mem = interop::get_inner::<Memory>(&env, this)?;
    Ok(mem.data_size() as jlong)
}

#[no_mangle]
extern "system" fn Java_wasmtime_Memory_dataSize(env: JNIEnv, this: JObject) -> jlong {
    wrap_error!(env, mem_data_size(&env, this))
}

fn mem_size(env: &JNIEnv, this: JObject) -> Result<jint> {
    let mem = interop::get_inner::<Memory>(&env, this)?;
    Ok(mem.size() as jint)
}

#[no_mangle]
extern "system" fn Java_wasmtime_Memory_size(env: JNIEnv, this: JObject) -> jint {
    wrap_error!(env, mem_size(&env, this))
}

fn mem_buffer(env: &JNIEnv, this: JObject) -> Result<jobject> {
    let mem = interop::get_inner::<Memory>(&env, this)?;
    let ptr = unsafe { mem.data_unchecked_mut() };
    Ok(env.new_direct_byte_buffer(ptr)?.into_inner())
}

#[no_mangle]
extern "system" fn Java_wasmtime_Memory_buffer(env: JNIEnv, this: JObject) -> jobject {
    match mem_buffer(&env, this) {
        Ok(obj) => obj,
        Err(e) => {
            errors::error_to_exception(&env, e);
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
extern "system" fn Java_wasmtime_Memory_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, interop::take_inner::<Memory>(&env, this));
}
