// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniFuncImpl;
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

trait JniFunc<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn native_call(
        env: &JNIEnv,
        this: JObject,
        args: jobjectArray,
    ) -> Result<jobjectArray, Self::Error>;
    fn new_func(
        env: &JNIEnv,
        clazz: JClass,
        store_ptr: jlong,
        fn_type: JObject,
        index: jint,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Func_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniFuncImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Func_nativeCall(
    env: JNIEnv,
    this: JObject,
    args: jobjectArray,
) -> jobjectArray {
    wrap_error!(
        env,
        JniFuncImpl::native_call(&env, this, args),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Func_newFunc(
    env: JNIEnv,
    clazz: JClass,
    store_ptr: jlong,
    fn_type: JObject,
    index: jint,
) -> jlong {
    wrap_error!(
        env,
        JniFuncImpl::new_func(&env, clazz, store_ptr, fn_type, index),
        Default::default()
    )
}
