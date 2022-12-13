use crate::wtrap;
use anyhow;
use jni::descriptors::Desc;
use jni::objects::JThrowable;
use jni::{self, JNIEnv};
use std::io;
use thiserror::Error;
use wasi_common::StringArrayError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JNI error: {0}")]
    Jni(#[from] jni::errors::Error),
    #[error("Wasmtime error: {0}")]
    Wasmtime(#[from] anyhow::Error),
    #[error("aborted instruction execution: {0}")]
    WasmTrap(#[from] wasmtime::Trap),
    #[error("wasi exit code: {0}")]
    WasiI32ExitCode(i32),
    #[error("unknown enum variant: {0}")]
    UnknownEnum(String),
    #[error("not implemented")]
    NotImplemented,
    #[error("{0}")]
    LockPoison(String),
    #[error("{0}")]
    WasiConfig(#[from] StringArrayError),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

impl<G> From<std::sync::PoisonError<G>> for Error {
    fn from(err: std::sync::PoisonError<G>) -> Self {
        Error::LockPoison(err.to_string())
    }
}

impl<'a> Desc<'a, JThrowable<'a>> for Error {
    fn lookup(self, env: &JNIEnv<'a>) -> jni::errors::Result<JThrowable<'a>> {
        use Error::*;
        let (ex_class, msg) = match &self {
            Jni(e) => {
                use jni::errors::Error::*;
                match e {
                    JavaException => return env.exception_occurred(),
                    NullPtr(_) | NullDeref(_) => {
                        ("java/lang/NullPointerException", self.to_string())
                    }
                    _ => (
                        "java/lang/RuntimeException",
                        format!("unknown exception caught (likely a BUG): {}", self),
                    ),
                }
            }
            Wasmtime(e) => (
                "io/github/kawamuray/wasmtime/WasmtimeException",
                e.to_string(),
            ),
            WasmTrap(trap) => {
                let jtrap = wtrap::into_java(env, trap)?;
                let jtrap_ex = env.new_object(
                    "io/github/kawamuray/wasmtime/WasmFunctionError$TrapError",
                    "(Lio/github/kawamuray/wasmtime/Trap;)V",
                    &[jtrap.into()],
                )?;
                return Ok(jtrap_ex.into());
            }
            WasiI32ExitCode(exit_code) => {
                return Ok(env
                    .new_object(
                        "io/github/kawamuray/wasmtime/WasmFunctionError$I32ExitError",
                        "(I)V",
                        &[(*exit_code).into()],
                    )?
                    .into())
            }
            WasiConfig(e) => (
                "io/github/kawamuray/wasmtime/WasmtimeException",
                e.to_string(),
            ),
            Io(_) | UnknownEnum(_) | NotImplemented | LockPoison(_) => {
                ("java/lang/RuntimeException", self.to_string())
            }
        };

        let jmsg = env.new_string(msg)?;
        Ok(env
            .new_object(ex_class, "(Ljava/lang/String;)V", &[jmsg.into()])?
            .into())
    }
}
