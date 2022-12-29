// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniCallerImpl;
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

trait JniCaller<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn data(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error>;
    fn native_get_export(
        env: &JNIEnv,
        this: JObject,
        name: JString,
    ) -> Result<jobject, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Caller_data(
    env: JNIEnv,
    this: JObject,
) -> jobject {
    wrap_error!(
        env,
        JniCallerImpl::data(&env, this),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Caller_nativeGetExport__Ljava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    name: JString,
) -> jobject {
    wrap_error!(
        env,
        JniCallerImpl::native_get_export(&env, this, name),
        JObject::null().into_raw()
    )
}
