use super::JniWasiCtx;
use crate::errors;
use crate::interop;
use crate::store::StoreData;
use crate::utils;
use crate::wasi_utils;
use jni::objects::*;
use jni::sys::*;
use jni::{self, JNIEnv};
use std::path::PathBuf;
use wasi_common::dir::DirCaps;
use wasi_common::file::FileCaps;
use wasmtime::Linker;
use wasmtime_wasi::WasiCtx;

pub(super) struct JniWasiCtxImpl;

impl<'a> JniWasiCtx<'a> for JniWasiCtxImpl {
    type Error = errors::Error;

    fn native_add_to_linker(
        _env: &mut JNIEnv<'a>,
        _clazz: JClass<'a>,
        linker_ptr: jlong,
    ) -> Result<(), Self::Error> {
        let mut linker = interop::ref_from_raw::<Linker<StoreData>>(linker_ptr)?;
        wasmtime_wasi::add_to_linker(&mut linker, |s| {
            &mut *s.wasi.as_mut().expect("WasiCtx in store must not empty")
        })?;
        Ok(())
    }

    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error> {
        interop::dispose_inner::<WasiCtx>(env, &this)?;
        Ok(())
    }

    fn native_insert_dir(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        fd: jint,
        dir_path: JString<'a>,
        dir_cap_bits: jint,
        file_cap_bits: jint,
        preopen_path: JString<'a>,
    ) -> Result<(), Self::Error> {
        let ctx = interop::get_inner::<WasiCtx>(env, &this)?;
        let wasi_dir = wasi_utils::open_wasi_dir(utils::get_string(env, &dir_path)?)?;
        ctx.insert_dir(
            fd as u32,
            Box::new(wasi_dir),
            DirCaps::from_bits_truncate(dir_cap_bits as u32),
            FileCaps::from_bits_truncate(file_cap_bits as u32),
            PathBuf::from(utils::get_string(env, &preopen_path)?),
        );
        Ok(())
    }

    fn native_insert_file(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        fd: jint,
        path: JString<'a>,
        file_cap_bits: jint,
    ) -> Result<(), Self::Error> {
        let ctx = interop::get_inner::<WasiCtx>(env, &this)?;
        let wasi_file = wasi_utils::open_wasi_file(utils::get_string(env, &path)?)?;
        ctx.insert_file(
            fd as u32,
            Box::new(wasi_file),
            FileCaps::from_bits_truncate(file_cap_bits as u32),
        );
        Ok(())
    }

    fn native_push_preopen_dir(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        dir_path: JString<'a>,
        path: JString<'a>,
    ) -> Result<(), Self::Error> {
        let ctx = interop::get_inner::<WasiCtx>(env, &this)?;
        let wasi_dir = wasi_utils::open_wasi_dir(&utils::get_string(env, &dir_path)?)?;
        ctx.push_preopened_dir(Box::new(wasi_dir), utils::get_string(env, &path)?)?;
        Ok(())
    }

    fn native_set_stderr(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        path: JString<'a>,
    ) -> Result<(), Self::Error> {
        let ctx = interop::get_inner::<WasiCtx>(env, &this)?;
        let path = utils::get_string(env, &path)?;
        ctx.set_stderr(Box::new(wasi_utils::open_wasi_file(path)?));
        Ok(())
    }

    fn native_set_stdin(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        path: JString<'a>,
    ) -> Result<(), Self::Error> {
        let ctx = interop::get_inner::<WasiCtx>(env, &this)?;
        let path = utils::get_string(env, &path)?;
        ctx.set_stdin(Box::new(wasi_utils::open_wasi_file(path)?));
        Ok(())
    }

    fn native_set_stdout(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        path: JString<'a>,
    ) -> Result<(), Self::Error> {
        let ctx = interop::get_inner::<WasiCtx>(env, &this)?;
        let path = utils::get_string(env, &path)?;
        ctx.set_stdout(Box::new(wasi_utils::open_wasi_file(path)?));
        Ok(())
    }

    fn push_arg(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        arg: JString<'a>,
    ) -> Result<(), Self::Error> {
        let mut ctx = interop::get_inner::<WasiCtx>(env, &this)?;
        ctx.push_arg(&utils::get_string(env, &arg)?)?;
        Ok(())
    }

    fn push_env(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        var: JString<'a>,
        value: JString<'a>,
    ) -> Result<(), Self::Error> {
        let mut ctx = interop::get_inner::<WasiCtx>(env, &this)?;
        ctx.push_env(
            &utils::get_string(env, &var)?,
            &utils::get_string(env, &value)?,
        )?;
        Ok(())
    }
}
