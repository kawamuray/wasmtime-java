// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniEngineImpl;
use jni::descriptors::Desc;
use jni::objects::*;
use jni::sys::*;
use jni::JNIEnv;

macro_rules! wrap_error {
    ($env:expr, $body:expr, $default:expr) => {
        match $body {
            Ok(v) => v,
            Err(e) => {
                if let Err(err) = $env.throw(e) {
                    $env.exception_describe().ok();
                    panic!("error in throwing exception: {}", err);
                }
                $default
            }
        }
    };
}

trait JniEngine<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn increment_epoch(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn new_engine(env: &JNIEnv, clazz: JClass) -> Result<jlong, Self::Error>;
    fn new_engine_with_config(
        env: &JNIEnv,
        clazz: JClass,
        config: JObject,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Engine_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniEngineImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Engine_incrementEpoch(
    env: JNIEnv,
    this: JObject,
) {
    wrap_error!(
        env,
        JniEngineImpl::increment_epoch(&env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Engine_newEngine(
    env: JNIEnv,
    clazz: JClass,
) -> jlong {
    wrap_error!(
        env,
        JniEngineImpl::new_engine(&env, clazz),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Engine_newEngineWithConfig__Lio_github_kawamuray_wasmtime_Config_2(
    env: JNIEnv,
    clazz: JClass,
    config: JObject,
) -> jlong {
    wrap_error!(
        env,
        JniEngineImpl::new_engine_with_config(&env, clazz, config),
        Default::default()
    )
}
