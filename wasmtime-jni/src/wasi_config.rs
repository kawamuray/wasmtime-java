use crate::errors::Result;
use crate::utils;
use jni::objects::JObject;
use jni::JNIEnv;
use std::fs::File;
use std::convert::TryFrom;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

fn preopen_dir(path: &str) -> Result<File> {
    Ok(File::open(path)?)
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
    builder.inherit_stdio().inherit_env().args(args);

    let iter = utils::JavaArrayIter::new(env, preopen_dirs.into_inner())?;
    for obj in iter {
        let obj = obj?;
        let host_path = utils::get_string_field(env, obj, "hostPath")?;
        let guest_path = utils::get_string_field(env, obj, "guestPath")?;
        let dir = preopen_dir(&host_path)?;
        builder.preopened_dir(dir, &guest_path);
    }

    if let Some(file) = utils::get_nullable_string_field(env, obj, "stdoutFile")? {
        let file = File::create(file)?;
        let file = wasi_common::OsFile::try_from(file)?;
        builder.stdout(file);
    }
    if let Some(file) = utils::get_nullable_string_field(env, obj, "stderrFile")? {
        let file = File::create(file)?;
        let file = wasi_common::OsFile::try_from(file)?;
        builder.stderr(file);
    }
    Ok(builder.build()?)
}
