use crate::errors::Result;
use crate::utils;
use cap_std::fs::Dir;
use jni::objects::JObject;
use jni::JNIEnv;
use std::fs::File;
use std::path::Path;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

fn preopen_dir(path: &str) -> Result<Dir> {
    Ok(unsafe { Dir::open_ambient_dir(path)? })
}

fn open_wasi_file<P: AsRef<Path>>(path: P) -> Result<wasi_cap_std_sync::file::File> {
    let file = File::create(path)?;
    let file = unsafe { cap_std::fs::File::from_std(file) };
    Ok(wasi_cap_std_sync::file::File::from_cap_std(file))
}

pub fn ctx_from_java(env: &JNIEnv, obj: JObject) -> Result<WasiCtx> {
    // TODO: incomplete implementation
    let args = env.get_field(obj, "args", "[Ljava/lang/String;")?.l()?;
    let iter = utils::JavaArrayIter::new(env, args.into_inner())?;
    let mut args = Vec::with_capacity(iter.len());
    for s in iter {
        args.push(utils::get_string(env, s?)?);
    }
    let preopen_dirs = env
        .get_field(
            obj,
            "preopenDirs",
            "[Lio/github/kawamuray/wasmtime/wasi/WasiConfig$PreopenDir;",
        )?
        .l()?;
    let mut builder = WasiCtxBuilder::new();
    builder = builder.inherit_stdio().inherit_env()?.args(&args)?;

    let iter = utils::JavaArrayIter::new(env, preopen_dirs.into_inner())?;
    for obj in iter {
        let obj = obj?;
        let host_path = utils::get_string_field(env, obj, "hostPath")?;
        let guest_path = utils::get_string_field(env, obj, "guestPath")?;
        let dir = preopen_dir(&host_path)?;
        builder = builder.preopened_dir(dir, &guest_path)?
    }

    if let Some(file) = utils::get_nullable_string_field(env, obj, "stdoutFile")? {
        let file = open_wasi_file(file)?;
        builder = builder.stdout(Box::new(file));
    }
    if let Some(file) = utils::get_nullable_string_field(env, obj, "stderrFile")? {
        let file = open_wasi_file(file)?;
        builder = builder.stderr(Box::new(file));
    }
    Ok(builder.build())
}
