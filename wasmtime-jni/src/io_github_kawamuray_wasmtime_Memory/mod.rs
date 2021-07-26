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
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn native_buffer(env: &JNIEnv, this: JObject, store_ptr: jlong)
        -> Result<jobject, Self::Error>;
    fn native_data_size(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
    ) -> Result<jlong, Self::Error>;
    fn native_grow(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
        delta_pages: jint,
    ) -> Result<jint, Self::Error>;
    fn native_size(env: &JNIEnv, this: JObject, store_ptr: jlong) -> Result<jint, Self::Error>;
    fn new_memory(
        env: &JNIEnv,
        clazz: JClass,
        store_ptr: jlong,
        min: jint,
        max: jint,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniMemoryImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_nativeBuffer(
    env: JNIEnv,
    this: JObject,
    store_ptr: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniMemoryImpl::native_buffer(&env, this, store_ptr),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_nativeDataSize(
    env: JNIEnv,
    this: JObject,
    store_ptr: jlong,
) -> jlong {
    wrap_error!(
        env,
        JniMemoryImpl::native_data_size(&env, this, store_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_nativeGrow(
    env: JNIEnv,
    this: JObject,
    store_ptr: jlong,
    delta_pages: jint,
) -> jint {
    wrap_error!(
        env,
        JniMemoryImpl::native_grow(&env, this, store_ptr, delta_pages),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_nativeSize(
    env: JNIEnv,
    this: JObject,
    store_ptr: jlong,
) -> jint {
    wrap_error!(
        env,
        JniMemoryImpl::native_size(&env, this, store_ptr),
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
