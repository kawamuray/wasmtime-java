use crate::errors::Result;
use crate::utils;
use jni::objects::JObject;
use jni::JNIEnv;
use std::fs::File;
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
        .get_field(obj, "preopenDirs", "[Lwasmtime/wasi/WasiConfig$PreopenDir;")?
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
    Ok(builder.build()?)
}
