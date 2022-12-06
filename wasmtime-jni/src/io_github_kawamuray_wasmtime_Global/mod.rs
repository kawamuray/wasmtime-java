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
                $env.throw(e).expect("error in throwing exception");
                $default
            }
        }
    };
}

trait JniGlobal<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn native_get(env: &JNIEnv, this: JObject, store_ptr: jlong) -> Result<jobject, Self::Error>;
    fn native_set(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
        val: JObject,
    ) -> Result<(), Self::Error>;
    fn native_mutable(env: &JNIEnv, this: JObject, store_ptr: jlong) -> Result<u8, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Global_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniGlobalImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Global_nativeGet(
    env: JNIEnv,
    this: JObject,
    store_ptr: jlong,
) -> jobjectArray {
    wrap_error!(
        env,
        JniGlobalImpl::native_get(&env, this, store_ptr),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Global_nativeSet(
    env: JNIEnv,
    this: JObject,
    store_ptr: jlong,
    val: JObject,
) {
    wrap_error!(
        env,
        JniGlobalImpl::native_set(&env, this, store_ptr, val),
        ()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Global_nativeMutable(
    env: JNIEnv,
    this: JObject,
    store_ptr: jlong,
) -> jboolean {
    wrap_error!(env, JniGlobalImpl::native_mutable(&env, this, store_ptr), 0)
}
