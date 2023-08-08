use crate::{errors::Result, utils, wtrap};
use anyhow::anyhow;
use jni::{objects::JThrowable, JNIEnv};

pub fn from_java(env: &mut JNIEnv, throwable: JThrowable) -> Result<anyhow::Error> {
    Ok(
        if env.is_instance_of(
            &throwable,
            "io/github/kawamuray/wasmtime/WasmFunctionError$I32ExitError",
        )? {
            let exit_code = env.call_method(&throwable, "exitCode", "()I", &[])?.i()?;
            anyhow!(wasi_common::I32Exit(exit_code))
        } else if env.is_instance_of(
            &throwable,
            "io/github/kawamuray/wasmtime/WasmFunctionError$TrapError",
        )? {
            let jtrap = env
                .call_method(
                    &throwable,
                    "trap",
                    "()Lio/github/kawamuray/wasmtime/Trap;",
                    &[],
                )?
                .l()?;
            let trap = wtrap::from_java(env, jtrap)?;
            anyhow!(trap)
        } else {
            let message = env
                .call_method(throwable, "getMessage", "()Ljava/lang/String;", &[])?
                .l()?;
            anyhow!(utils::get_string(env, &message)?)
        },
    )
}
