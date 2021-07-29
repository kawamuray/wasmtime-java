// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniStoreImpl;
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

trait JniStore<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn engine_ptr(env: &JNIEnv, this: JObject) -> Result<jlong, Self::Error>;
    fn gc(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn new_store(
        env: &JNIEnv,
        clazz: JClass,
        engine_ptr: jlong,
        data: JObject,
        wasi_ctx_ptr: jlong,
    ) -> Result<jlong, Self::Error>;
    fn stored_data(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniStoreImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_enginePtr(
    env: JNIEnv,
    this: JObject,
) -> jlong {
    wrap_error!(
        env,
        JniStoreImpl::engine_ptr(&env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_gc(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniStoreImpl::gc(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_newStore(
    env: JNIEnv,
    clazz: JClass,
    engine_ptr: jlong,
    data: JObject,
    wasi_ctx_ptr: jlong,
) -> jlong {
    wrap_error!(
        env,
        JniStoreImpl::new_store(&env, clazz, engine_ptr, data, wasi_ctx_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_storedData(
    env: JNIEnv,
    this: JObject,
) -> jobject {
    wrap_error!(
        env,
        JniStoreImpl::stored_data(&env, this),
        JObject::null().into_inner()
    )
}
