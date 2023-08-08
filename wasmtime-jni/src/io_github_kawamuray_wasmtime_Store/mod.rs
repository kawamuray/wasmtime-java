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
                if let Err(err) = $env.throw(e) {
                    $env.exception_describe().ok();
                    panic!("error in throwing exception: {}", err);
                }
                $default
            }
        }
    };
}

trait JniStore<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn engine_ptr(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<jlong, Self::Error>;
    fn gc(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn new_store(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        engine_ptr: jlong,
        data: JObject<'a>,
        wasi_ctx_ptr: jlong,
    ) -> Result<jlong, Self::Error>;
    fn set_epoch_deadline(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        ticks_beyond_current: jlong,
    ) -> Result<(), Self::Error>;
    fn stored_data(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<jobject, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniStoreImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_enginePtr<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) -> jlong {
    wrap_error!(
        env,
        JniStoreImpl::engine_ptr(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_gc<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(env, JniStoreImpl::gc(&mut env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_newStore__JLjava_lang_Object_2J<'a>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    engine_ptr: jlong,
    data: JObject<'a>,
    wasi_ctx_ptr: jlong,
) -> jlong {
    wrap_error!(
        env,
        JniStoreImpl::new_store(&mut env, clazz, engine_ptr, data, wasi_ctx_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_setEpochDeadline__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    ticks_beyond_current: jlong,
) {
    wrap_error!(
        env,
        JniStoreImpl::set_epoch_deadline(&mut env, this, ticks_beyond_current),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Store_storedData<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) -> jobject {
    wrap_error!(
        env,
        JniStoreImpl::stored_data(&mut env, this),
        JObject::null().into_raw()
    )
}
