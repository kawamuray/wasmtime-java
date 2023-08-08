// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniWasiCtxBuilderImpl;
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

trait JniWasiCtxBuilder<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn native_build(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        envs: jobjectArray,
        args: jobjectArray,
        inherit_stdin: jboolean,
        stdin_path: JString<'a>,
        inherit_stdout: jboolean,
        stdout_path: JString<'a>,
        inherit_stderr: jboolean,
        stderr_path: JString<'a>,
        preopen_dirs: jobjectArray,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtxBuilder_nativeBuild___3Ljava_lang_Object_2_3Ljava_lang_Object_2ZLjava_lang_String_2ZLjava_lang_String_2ZLjava_lang_String_2_3Ljava_lang_Object_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    envs: jobjectArray,
    args: jobjectArray,
    inherit_stdin: jboolean,
    stdin_path: JString<'a>,
    inherit_stdout: jboolean,
    stdout_path: JString<'a>,
    inherit_stderr: jboolean,
    stderr_path: JString<'a>,
    preopen_dirs: jobjectArray,
) -> jlong {
    wrap_error!(
        env,
        JniWasiCtxBuilderImpl::native_build(
            &mut env,
            clazz,
            envs,
            args,
            inherit_stdin,
            stdin_path,
            inherit_stdout,
            stdout_path,
            inherit_stderr,
            stderr_path,
            preopen_dirs
        ),
        Default::default()
    )
}
