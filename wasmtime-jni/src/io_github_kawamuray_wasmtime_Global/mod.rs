// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniGlobalImpl;
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

trait JniGlobal<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn native_get(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jobject, Self::Error>;
    fn native_mutable(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jboolean, Self::Error>;
    fn native_set(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        val: JObject<'a>,
    ) -> Result<(), Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Global_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniGlobalImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Global_nativeGet__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniGlobalImpl::native_get(&mut env, this, store_ptr),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Global_nativeMutable__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
) -> jboolean {
    wrap_error!(
        env,
        JniGlobalImpl::native_mutable(&mut env, this, store_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Global_nativeSet__JLio_github_kawamuray_wasmtime_Val_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
    val: JObject<'a>,
) {
    wrap_error!(
        env,
        JniGlobalImpl::native_set(&mut env, this, store_ptr, val),
        Default::default()
    )
}
