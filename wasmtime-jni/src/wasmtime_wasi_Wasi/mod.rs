// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniWasiImpl;
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

trait JniWasi<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn native_add_to_linker(
        env: &JNIEnv,
        this: JObject,
        linker_ptr: jlong,
    ) -> Result<(), Self::Error>;
    fn new_wasi(
        env: &JNIEnv,
        clazz: JClass,
        store_ptr: jlong,
        config: JObject,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_wasmtime_wasi_Wasi_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniWasiImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_wasmtime_wasi_Wasi_nativeAddToLinker(
    env: JNIEnv,
    this: JObject,
    linker_ptr: jlong,
) {
    wrap_error!(
        env,
        JniWasiImpl::native_add_to_linker(&env, this, linker_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_wasmtime_wasi_Wasi_newWasi(
    env: JNIEnv,
    clazz: JClass,
    store_ptr: jlong,
    config: JObject,
) -> jlong {
    wrap_error!(
        env,
        JniWasiImpl::new_wasi(&env, clazz, store_ptr, config),
        Default::default()
    )
}
