use super::JniStore;
use crate::errors;
use crate::interop;
use crate::store::StoreData;
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::{self, JNIEnv};
use wasmtime::{Engine, Store};

pub(super) struct JniStoreImpl;

impl<'a> JniStore<'a> for JniStoreImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::take_inner::<Store<StoreData>>(&env, this)?;
        Ok(())
    }

    fn new_store(_env: &JNIEnv, _clazz: JClass, data: JObject) -> Result<jlong, Self::Error> {
        let store = Store::new(&Engine::default(), StoreData { wasi: None });
        Ok(interop::into_raw::<Store<_>>(store))
    }

    fn new_store_with_engine(
        env: &JNIEnv,
        _clazz: JClass,
        data: JObject,
        engine: JObject,
    ) -> Result<jlong, Self::Error> {
        // TODO: store ref to data
        let engine = interop::get_inner::<Engine>(&env, engine)?;
        let store = Store::new(&engine, StoreData { wasi: None });
        Ok(interop::into_raw::<Store<_>>(store))
    }

    fn engine_ptr(env: &JNIEnv, this: JObject) -> Result<jlong, Self::Error> {
        let store = interop::get_inner::<Store<StoreData>>(&env, this)?;
        let engine: Engine = store.engine().clone();
        Ok(interop::into_raw::<Engine>(engine))
    }

    fn stored_data(env: &JNIEnv, this: JObject) -> Result<jni::sys::jobject, Self::Error> {
        todo!()
    }
}
