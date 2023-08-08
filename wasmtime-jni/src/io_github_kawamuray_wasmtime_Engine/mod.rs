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
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn increment_epoch(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn new_engine(env: &mut JNIEnv<'a>, clazz: JClass<'a>) -> Result<jlong, Self::Error>;
    fn new_engine_with_config(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        config: JObject<'a>,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Engine_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniEngineImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Engine_incrementEpoch<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniEngineImpl::increment_epoch(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Engine_newEngine<'a>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
) -> jlong {
    wrap_error!(
        env,
        JniEngineImpl::new_engine(&mut env, clazz),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Engine_newEngineWithConfig__Lio_github_kawamuray_wasmtime_Config_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    config: JObject<'a>,
) -> jlong {
    wrap_error!(
        env,
        JniEngineImpl::new_engine_with_config(&mut env, clazz, config),
        Default::default()
    )
}
