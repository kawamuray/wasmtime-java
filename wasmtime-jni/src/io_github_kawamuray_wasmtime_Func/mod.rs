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
                if let Err(err) = $env.throw(e) {
                    $env.exception_describe().ok();
                    panic!("error in throwing exception: {}", err);
                }
                $default
            }
        }
    };
}

trait JniFunc<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn native_call(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        args: jobjectArray,
    ) -> Result<jobjectArray, Self::Error>;
    fn new_func(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        store_ptr: jlong,
        fn_type: JObject<'a>,
        index: jint,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Func_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniFuncImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Func_nativeCall__J_3Lio_github_kawamuray_wasmtime_Val_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
    args: jobjectArray,
) -> jobjectArray {
    wrap_error!(
        env,
        JniFuncImpl::native_call(&mut env, this, store_ptr, args),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Func_newFunc__JLio_github_kawamuray_wasmtime_FuncType_2I<
    'a,
>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    store_ptr: jlong,
    fn_type: JObject<'a>,
    index: jint,
) -> jlong {
    wrap_error!(
        env,
        JniFuncImpl::new_func(&mut env, clazz, store_ptr, fn_type, index),
        Default::default()
    )
}
