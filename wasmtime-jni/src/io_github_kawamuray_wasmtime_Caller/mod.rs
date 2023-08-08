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
    fn data(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<jobject, Self::Error>;
    fn native_get_export(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        name: JString<'a>,
    ) -> Result<jobject, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Caller_data<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) -> jobject {
    wrap_error!(
        env,
        JniCallerImpl::data(&mut env, this),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Caller_nativeGetExport__Ljava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    name: JString<'a>,
) -> jobject {
    wrap_error!(
        env,
        JniCallerImpl::native_get_export(&mut env, this, name),
        JObject::null().into_raw()
    )
}
