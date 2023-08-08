// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniModuleImpl;
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

trait JniModule<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn imports(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<jobjectArray, Self::Error>;
    fn new_from_binary(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        engine_ptr: jlong,
        bytes: jbyteArray,
    ) -> Result<jlong, Self::Error>;
    fn new_from_file(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        engine_ptr: jlong,
        file_name: JString<'a>,
    ) -> Result<jlong, Self::Error>;
    fn new_module(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        engine_ptr: jlong,
        bytes: jbyteArray,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniModuleImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_imports<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) -> jobjectArray {
    wrap_error!(
        env,
        JniModuleImpl::imports(&mut env, this),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_newFromBinary__J_3B<'a>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    engine_ptr: jlong,
    bytes: jbyteArray,
) -> jlong {
    wrap_error!(
        env,
        JniModuleImpl::new_from_binary(&mut env, clazz, engine_ptr, bytes),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_newFromFile__JLjava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    engine_ptr: jlong,
    file_name: JString<'a>,
) -> jlong {
    wrap_error!(
        env,
        JniModuleImpl::new_from_file(&mut env, clazz, engine_ptr, file_name),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_newModule__J_3B<'a>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    engine_ptr: jlong,
    bytes: jbyteArray,
) -> jlong {
    wrap_error!(
        env,
        JniModuleImpl::new_module(&mut env, clazz, engine_ptr, bytes),
        Default::default()
    )
}
