use super::JniWasiCtxBuilder;
use crate::errors;
use crate::errors::Result;
use crate::interop;
use crate::utils;
use crate::wasi_utils;
use jni::objects::*;
use jni::sys::*;
use jni::JNIEnv;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

pub(super) struct JniWasiCtxBuilderImpl;

impl<'a> JniWasiCtxBuilder<'a> for JniWasiCtxBuilderImpl {
    type Error = errors::Error;

    fn native_build(
        env: &JNIEnv,
        _class: JClass,
        envs: jobjectArray,
        args: jobjectArray,
        inherit_stdin: jboolean,
        stdin_path: JString,
        inherit_stdout: jboolean,
        stdout_path: JString,
        inherit_stderr: jboolean,
        stderr_path: JString,
        preopen_dirs: jobjectArray,
    ) -> Result<jlong, Self::Error> {
        let mut builder = WasiCtxBuilder::new();

        for pair in utils::JavaArrayIter::new(env, envs)? {
            let pair = pair?;
            let var = env.get_object_array_element(pair.into_raw(), 0)?;
            let value = env.get_object_array_element(pair.into_raw(), 1)?;

            builder = builder.env(
                &utils::get_string(env, var)?,
                &utils::get_string(env, value)?,
            )?;
        }
        for arg in utils::JavaArrayIter::new(env, args)? {
            builder = builder.arg(&utils::get_string(env, arg?)?)?;
        }

        if inherit_stdin != 0 {
            builder = builder.inherit_stdin();
        } else if !stdin_path.is_null() {
            let file = wasi_utils::open_wasi_file(utils::get_string(env, stdin_path.into())?)?;
            builder = builder.stdin(Box::new(file));
        }
        if inherit_stdout != 0 {
            builder = builder.inherit_stdout();
        } else if !stdout_path.is_null() {
            let file = wasi_utils::open_wasi_file(utils::get_string(env, stdout_path.into())?)?;
            builder = builder.stdout(Box::new(file));
        }
        if inherit_stderr != 0 {
            builder = builder.inherit_stderr();
        } else if !stderr_path.is_null() {
            let file = wasi_utils::open_wasi_file(utils::get_string(env, stderr_path.into())?)?;
            builder = builder.stderr(Box::new(file));
        }

        for obj in utils::JavaArrayIter::new(env, preopen_dirs)? {
            let obj = obj?;
            let host_path = utils::get_string_field(env, obj, "hostPath")?;
            let guest_path = utils::get_string_field(env, obj, "guestPath")?;
            let dir = wasi_utils::open_dir(&host_path)?;
            builder = builder.preopened_dir(dir, guest_path)?;
        }

        let ctx = builder.build();
        let ptr = interop::into_raw::<WasiCtx>(ctx);
        Ok(ptr)
    }
}
