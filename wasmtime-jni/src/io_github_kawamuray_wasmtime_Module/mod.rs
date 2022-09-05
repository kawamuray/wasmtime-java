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
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn imports(env: &JNIEnv, this: JObject) -> Result<jobjectArray, Self::Error>;
    fn new_from_binary(
        env: &JNIEnv,
        clazz: JClass,
        engine_ptr: jlong,
        bytes: jbyteArray,
    ) -> Result<jlong, Self::Error>;
    fn new_from_file(
        env: &JNIEnv,
        clazz: JClass,
        engine_ptr: jlong,
        file_name: JString,
    ) -> Result<jlong, Self::Error>;
    fn new_module(
        env: &JNIEnv,
        clazz: JClass,
        engine_ptr: jlong,
        bytes: jbyteArray,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniModuleImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_imports(
    env: JNIEnv,
    this: JObject,
) -> jobjectArray {
    wrap_error!(
        env,
        JniModuleImpl::imports(&env, this),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_newFromBinary(
    env: JNIEnv,
    clazz: JClass,
    engine_ptr: jlong,
    bytes: jbyteArray,
) -> jlong {
    wrap_error!(
        env,
        JniModuleImpl::new_from_binary(&env, clazz, engine_ptr, bytes),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_newFromFile(
    env: JNIEnv,
    clazz: JClass,
    engine_ptr: jlong,
    file_name: JString,
) -> jlong {
    wrap_error!(
        env,
        JniModuleImpl::new_from_file(&env, clazz, engine_ptr, file_name),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Module_newModule(
    env: JNIEnv,
    clazz: JClass,
    engine_ptr: jlong,
    bytes: jbyteArray,
) -> jlong {
    wrap_error!(
        env,
        JniModuleImpl::new_module(&env, clazz, engine_ptr, bytes),
        Default::default()
    )
}
