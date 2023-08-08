// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniInstanceImpl;
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

trait JniInstance<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn native_get_func(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        name: JString<'a>,
    ) -> Result<jlong, Self::Error>;
    fn native_get_memory(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        name: JString<'a>,
    ) -> Result<jlong, Self::Error>;
    fn new_instance(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        store_ptr: jlong,
        module_ptr: jlong,
        externs: jobjectArray,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Instance_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniInstanceImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Instance_nativeGetFunc__JLjava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
    name: JString<'a>,
) -> jlong {
    wrap_error!(
        env,
        JniInstanceImpl::native_get_func(&mut env, this, store_ptr, name),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Instance_nativeGetMemory__JLjava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
    name: JString<'a>,
) -> jlong {
    wrap_error!(
        env,
        JniInstanceImpl::native_get_memory(&mut env, this, store_ptr, name),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Instance_newInstance__JJ_3Lio_github_kawamuray_wasmtime_Extern_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    store_ptr: jlong,
    module_ptr: jlong,
    externs: jobjectArray,
) -> jlong {
    wrap_error!(
        env,
        JniInstanceImpl::new_instance(&mut env, clazz, store_ptr, module_ptr, externs),
        Default::default()
    )
}
