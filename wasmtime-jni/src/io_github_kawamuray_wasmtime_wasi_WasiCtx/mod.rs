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
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn native_add_to_linker(
        env: &JNIEnv,
        clazz: JClass,
        linker_ptr: jlong,
    ) -> Result<(), Self::Error>;
    fn native_insert_dir(
        env: &JNIEnv,
        this: JObject,
        fd: jint,
        dir_path: JString,
        dir_cap_bits: jint,
        file_cap_bits: jint,
        preopen_path: JString,
    ) -> Result<(), Self::Error>;
    fn native_insert_file(
        env: &JNIEnv,
        this: JObject,
        fd: jint,
        path: JString,
        file_cap_bits: jint,
    ) -> Result<(), Self::Error>;
    fn native_push_preopen_dir(
        env: &JNIEnv,
        this: JObject,
        dir_path: JString,
        path: JString,
    ) -> Result<(), Self::Error>;
    fn native_set_stderr(env: &JNIEnv, this: JObject, path: JString) -> Result<(), Self::Error>;
    fn native_set_stdin(env: &JNIEnv, this: JObject, path: JString) -> Result<(), Self::Error>;
    fn native_set_stdout(env: &JNIEnv, this: JObject, path: JString) -> Result<(), Self::Error>;
    fn push_arg(env: &JNIEnv, this: JObject, arg: JString) -> Result<(), Self::Error>;
    fn push_env(
        env: &JNIEnv,
        this: JObject,
        var: JString,
        value: JString,
    ) -> Result<(), Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_dispose(
    env: JNIEnv,
    this: JObject,
) {
    wrap_error!(env, JniWasiCtxImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeAddToLinker__J(
    env: JNIEnv,
    clazz: JClass,
    linker_ptr: jlong,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_add_to_linker(&env, clazz, linker_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeInsertDir__ILjava_lang_String_2IILjava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    fd: jint,
    dir_path: JString,
    dir_cap_bits: jint,
    file_cap_bits: jint,
    preopen_path: JString,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_insert_dir(
            &env,
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
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeInsertFile__ILjava_lang_String_2I(
    env: JNIEnv,
    this: JObject,
    fd: jint,
    path: JString,
    file_cap_bits: jint,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_insert_file(&env, this, fd, path, file_cap_bits),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativePushPreopenDir__Ljava_lang_String_2Ljava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    dir_path: JString,
    path: JString,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_push_preopen_dir(&env, this, dir_path, path),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeSetStderr__Ljava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    path: JString,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_set_stderr(&env, this, path),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeSetStdin__Ljava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    path: JString,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_set_stdin(&env, this, path),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_nativeSetStdout__Ljava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    path: JString,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::native_set_stdout(&env, this, path),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_pushArg__Ljava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    arg: JString,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::push_arg(&env, this, arg),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_wasi_WasiCtx_pushEnv__Ljava_lang_String_2Ljava_lang_String_2(
    env: JNIEnv,
    this: JObject,
    var: JString,
    value: JString,
) {
    wrap_error!(
        env,
        JniWasiCtxImpl::push_env(&env, this, var, value),
        Default::default()
    )
}
