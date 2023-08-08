// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniWasiCtxImpl;
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

trait JniWasiCtx<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn native_add_to_linker(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        linker_ptr: jlong,
    ) -> Result<(), Self::Error>;
    fn native_insert_dir(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        fd: jint,
        dir_path: JString<'a>,
        dir_cap_bits: jint,
        file_cap_bits: jint,
        preopen_path: JString<'a>,
    ) -> Result<(), Self::Error>;
    fn native_insert_file(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        fd: jint,
        path: JString<'a>,
        file_cap_bits: jint,
    ) -> Result<(), Self::Error>;
    fn native_push_preopen_dir(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        dir_path: JString<'a>,
        path: JString<'a>,
    ) -> Result<(), Self::Error>;
    fn native_set_stderr(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        path: JString<'a>,
    ) -> Result<(), Self::Error>;
    fn native_set_stdin(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        path: JString<'a>,
    ) -> Result<(), Self::Error>;
    fn native_set_stdout(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        path: JString<'a>,
    ) -> Result<(), Self::Error>;
    fn push_arg(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        arg: JString<'a>,
    ) -> Result<(), Self::Error>;
    fn push_env(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        var: JString<'a>,
        value: JString<'a>,
    ) -> Result<(), Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeAddToLinker__J<'a>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    linker_ptr: jlong,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_add_to_linker(&mut env, clazz, linker_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeInsertDir__ILjava_lang_String_2IILjava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    fd: jint,
    dir_path: JString<'a>,
    dir_cap_bits: jint,
    file_cap_bits: jint,
    preopen_path: JString<'a>,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_insert_dir(
            &mut env,
            this,
            fd,
            dir_path,
            dir_cap_bits,
            file_cap_bits,
            preopen_path
        ),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeInsertFile__ILjava_lang_String_2I<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    fd: jint,
    path: JString<'a>,
    file_cap_bits: jint,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_insert_file(&mut env, this, fd, path, file_cap_bits),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativePushPreopenDir__Ljava_lang_String_2Ljava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    dir_path: JString<'a>,
    path: JString<'a>,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_push_preopen_dir(&mut env, this, dir_path, path),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeSetStderr__Ljava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    path: JString<'a>,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_set_stderr(&mut env, this, path),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeSetStdin__Ljava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    path: JString<'a>,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_set_stdin(&mut env, this, path),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeSetStdout__Ljava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    path: JString<'a>,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_set_stdout(&mut env, this, path),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_pushArg__Ljava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    arg: JString<'a>,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::push_arg(&mut env, this, arg),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_pushEnv__Ljava_lang_String_2Ljava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    var: JString<'a>,
    value: JString<'a>,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::push_env(&mut env, this, var, value),
        Default::default()
    )
}
