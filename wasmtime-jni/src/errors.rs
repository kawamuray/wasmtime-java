use anyhow;
use jni::{self, JNIEnv};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("JNI error: {0}")]
    Jni(#[from] jni::errors::Error),
    #[error("Wasmtime error: {0}")]
    Wasmtime(#[from] anyhow::Error),
    #[error("unknown enum variant: {0}")]
    UnknownEnum(String),
    #[error("not implemented")]
    NotImplemented,
    #[error("{0}")]
    LockPoison(String),
}

impl<G> From<std::sync::PoisonError<G>> for Error {
    fn from(err: std::sync::PoisonError<G>) -> Self {
        Error::LockPoison(err.to_string())
    }
}

pub fn throw_exception(env: &JNIEnv, msg: &str) {
    env.throw_new("java/lang/RuntimeException", msg)
        .expect("failed throwing RuntimeException");
}

/// AAAA
/// Apparently jni-rs has `ToException` trait that supposably prepared
/// for this purpose but it doesn't seems to be implemented.
pub fn jni_error_to_exception(env: &JNIEnv, err: jni::errors::Error) {
    use jni::errors::ErrorKind::*;
    match err.kind() {
        JavaException => {
            let exception = env
                .exception_occurred()
                .expect("failed obtaining exception");
            env.throw(exception).expect("failed throwing exception");
        }
        NullPtr(_) | NullDeref(_) => env
            .throw_new("java/lang/NullPointerException", err.to_string())
            .expect("failed throwing NullPointerException"),
        _ => throw_exception(
            &env,
            &format!("unknown exception caught (likely a BUG): {}", err),
        ),
    }
}

pub fn wasmtime_error_to_exception(env: &JNIEnv, err: anyhow::Error) {
    env.throw_new("wasmtime/WasmtimeException", err.to_string())
        .expect("failed throwing exception");
}

pub fn error_to_exception(env: &JNIEnv, err: Error) {
    use Error::*;
    match err {
        Jni(e) => jni_error_to_exception(env, e),
        Wasmtime(e) => wasmtime_error_to_exception(env, e),
        UnknownEnum(_) | NotImplemented | LockPoison(_) => throw_exception(env, &err.to_string()),
    }
}

pub fn return_error<T, E>(env: &JNIEnv, err: E) -> T
where
    T: Default,
    E: Into<Error>,
{
    error_to_exception(env, err.into());
    T::default()
}

#[macro_export]
macro_rules! wrap_error {
    ($env:expr, $body:expr) => {
        match $body {
            Ok(v) => v,
            Err(e) => return crate::errors::return_error(&$env, e),
        }
    };
}
