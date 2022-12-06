use super::JniStore;
use crate::errors;
use crate::interop;
use crate::store::StoreData;
use jni::objects::{JClass, JObject};
use jni::sys::*;
use jni::{self, JNIEnv};
use wasmtime::{Engine, Store};
use wasmtime_wasi::WasiCtx;

pub(super) struct JniStoreImpl;

impl<'a> JniStore<'a> for JniStoreImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::dispose_inner::<Store<StoreData>>(&env, this)?;
        Ok(())
    }

    fn new_store(
        env: &JNIEnv,
        _clazz: JClass,
        engine_ptr: jlong,
        data: JObject,
        wasi_ctx_ptr: jlong,
    ) -> Result<jlong, Self::Error> {
        let engine = interop::ref_from_raw::<Engine>(engine_ptr)?.clone();
        let wasi = if wasi_ctx_ptr == 0 {
            None
        } else {
            Some(interop::from_raw::<WasiCtx>(wasi_ctx_ptr)?)
        };
        let java_data = if data.is_null() {
            None
        } else {
            Some(env.new_global_ref(data)?)
        };
        let store = Store::new(&engine, StoreData { wasi, java_data });
        Ok(interop::into_raw::<Store<_>>(store))
    }

    fn engine_ptr(env: &JNIEnv, this: JObject) -> Result<jlong, Self::Error> {
        let store = interop::get_inner::<Store<StoreData>>(&env, this)?;
        let engine = store.engine().clone();
        Ok(interop::into_raw::<Engine>(engine))
    }

    fn stored_data(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error> {
        let store = interop::get_inner::<Store<StoreData>>(&env, this)?;
        let data = store.data();
        Ok(if let Some(gref) = &data.java_data {
            gref.as_obj().into_raw()
        } else {
            JObject::null().into_raw()
        })
    }

    fn gc(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        let mut store = interop::get_inner::<Store<StoreData>>(&env, this)?;
        store.gc();
        Ok(())
    }

    fn set_epoch_deadline(
        env: &JNIEnv,
        this: JObject,
        ticks_beyond_current: jlong,
    ) -> Result<(), Self::Error> {
        let mut store = interop::get_inner::<Store<StoreData>>(&env, this)?;
        store.set_epoch_deadline(ticks_beyond_current as u64);
        Ok(())
    }
}
