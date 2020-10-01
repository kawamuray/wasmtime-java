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
    fn new_store(env: &JNIEnv, clazz: JClass) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_wasmtime_Store_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniStoreImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_wasmtime_Store_enginePtr(env: JNIEnv, this: JObject) -> jlong {
    wrap_error!(
        env,
        JniStoreImpl::engine_ptr(&env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_wasmtime_Store_newStore(env: JNIEnv, clazz: JClass) -> jlong {
    wrap_error!(
        env,
        JniStoreImpl::new_store(&env, clazz),
        Default::default()
    )
}
