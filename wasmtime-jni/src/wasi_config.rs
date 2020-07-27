use crate::errors::Result;
use crate::utils;
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime_wasi::WasiCtx;

pub fn ctx_from_java(env: &JNIEnv, obj: JObject) -> Result<WasiCtx> {
    // TODO: incomplete implementation
    let args = env.get_field(obj, "args", "[Ljava/lang/String;")?.l()?;
    let iter = utils::JavaArrayIter::new(env, args.into_inner())?;
    let mut strings = Vec::with_capacity(iter.len());
    for s in iter {
        strings.push(utils::get_string(env, s?)?);
    }
    Ok(WasiCtx::new(strings)?)
}
