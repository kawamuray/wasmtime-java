use super::JniEngine;
use crate::errors;
use crate::interop;
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::{self, JNIEnv};
use wasmtime::{Config, Engine};

pub(super) struct JniEngineImpl;

impl<'a> JniEngine<'a> for JniEngineImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::take_inner::<Engine>(&env, this)?;
        Ok(())
    }

    fn new_engine(_env: &JNIEnv, _clazz: JClass) -> Result<jlong, Self::Error> {
        let engine = Engine::default();
        Ok(interop::into_raw::<Engine>(engine))
    }

    fn new_engine_with_config(
        _env: &JNIEnv,
        _clazz: JClass,
        config: JObject,
    ) -> Result<jlong, Self::Error> {
        let config = interop::take_inner::<Config>(&_env, config)?;
        let engine = Engine::new(&config);
        Ok(interop::into_raw::<Engine>(engine))
    }
}
