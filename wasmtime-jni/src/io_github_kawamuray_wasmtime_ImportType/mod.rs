// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniImportTypeImpl;
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

trait JniImportType<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn module(env: &JNIEnv, this: JObject) -> Result<jstring, Self::Error>;
    fn name(env: &JNIEnv, this: JObject) -> Result<jstring, Self::Error>;
    fn ty(env: &JNIEnv, this: JObject) -> Result<jstring, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_ImportType_dispose(
    env: JNIEnv,
    this: JObject,
) {
    wrap_error!(
        env,
        JniImportTypeImpl::dispose(&env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_ImportType_module(
    env: JNIEnv,
    this: JObject,
) -> jstring {
    wrap_error!(
        env,
        JniImportTypeImpl::module(&env, this),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_ImportType_name(
    env: JNIEnv,
    this: JObject,
) -> jstring {
    wrap_error!(
        env,
        JniImportTypeImpl::name(&env, this),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_ImportType_ty(
    env: JNIEnv,
    this: JObject,
) -> jstring {
    wrap_error!(
        env,
        JniImportTypeImpl::ty(&env, this),
        JObject::null().into_inner()
    )
}
