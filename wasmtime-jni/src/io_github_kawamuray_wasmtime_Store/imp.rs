use super::JniStore;
use crate::errors;
use crate::interop;
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::{self, JNIEnv};
use wasmtime::{Engine, Store};

pub(super) struct JniStoreImpl;

impl<'a> JniStore<'a> for JniStoreImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::take_inner::<Store>(&env, this)?;
        Ok(())
    }

    fn new_store(_env: &JNIEnv, _clazz: JClass) -> Result<jlong, Self::Error> {
        let store = Store::default();
        Ok(interop::into_raw::<Store>(store))
    }

    fn engine_ptr(env: &JNIEnv, this: JObject) -> Result<jlong, Self::Error> {
        let store = interop::get_inner::<Store>(&env, this)?;
        let engine: Engine = store.engine().clone();
        Ok(interop::into_raw::<Engine>(engine))
    }
}
