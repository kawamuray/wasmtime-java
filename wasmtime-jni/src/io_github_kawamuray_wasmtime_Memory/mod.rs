// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniMemoryImpl;
use jni::descriptors::Desc;
use jni::objects::*;
use jni::sys::*;
use jni::JNIEnv;

macro_rules! wrap_error {
    ($env:expr, $body:expr, $default:expr) => {
        match $body {
            Ok(v) => v,
            Err(e) => {
                $env.throw(e).expect("error in throwing exception");
                $default
            }
        }
    };
}

trait JniMemory<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn buffer(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error>;
    fn data_size(env: &JNIEnv, this: JObject) -> Result<jlong, Self::Error>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn grow(env: &JNIEnv, this: JObject, delta_pages: jint) -> Result<jint, Self::Error>;
    fn new_memory(
        env: &JNIEnv,
        clazz: JClass,
        store_ptr: jlong,
        min: jint,
        max: jint,
    ) -> Result<jlong, Self::Error>;
    fn size(env: &JNIEnv, this: JObject) -> Result<jint, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_buffer(
    env: JNIEnv,
    this: JObject,
) -> jobject {
    wrap_error!(
        env,
        JniMemoryImpl::buffer(&env, this),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_dataSize(
    env: JNIEnv,
    this: JObject,
) -> jlong {
    wrap_error!(
        env,
        JniMemoryImpl::data_size(&env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniMemoryImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_grow(
    env: JNIEnv,
    this: JObject,
    delta_pages: jint,
) -> jint {
    wrap_error!(
        env,
        JniMemoryImpl::grow(&env, this, delta_pages),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_newMemory(
    env: JNIEnv,
    clazz: JClass,
    store_ptr: jlong,
    min: jint,
    max: jint,
) -> jlong {
    wrap_error!(
        env,
        JniMemoryImpl::new_memory(&env, clazz, store_ptr, min, max),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_size(
    env: JNIEnv,
    this: JObject,
) -> jint {
    wrap_error!(env, JniMemoryImpl::size(&env, this), Default::default())
}
