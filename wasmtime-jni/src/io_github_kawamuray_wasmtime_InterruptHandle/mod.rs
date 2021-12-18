// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniInterruptHandleImpl;
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

trait JniInterruptHandle<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn interrupt(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_InterruptHandle_interrupt(
    env: JNIEnv,
    this: JObject,
) {
    wrap_error!(
        env,
        JniInterruptHandleImpl::interrupt(&env, this),
        Default::default()
    )
}
