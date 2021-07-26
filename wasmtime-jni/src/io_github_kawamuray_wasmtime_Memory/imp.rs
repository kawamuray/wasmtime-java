use super::JniMemory;
use crate::errors::{self, Result};
use crate::interop;
use crate::store::StoreData;
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong, jobject};
use jni::JNIEnv;
use wasmtime::{Limits, Memory, MemoryType, Store};

pub(super) struct JniMemoryImpl;

impl<'a> JniMemory<'a> for JniMemoryImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::dispose_inner::<Memory>(&env, this)?;
        Ok(())
    }

    fn native_buffer(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let mem = interop::get_inner::<Memory>(&env, this)?;
        let ptr = mem.data_mut(&mut *store);
        Ok(env.new_direct_byte_buffer(ptr)?.into_inner())
    }

    fn native_data_size(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
    ) -> Result<jlong, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let mem = interop::get_inner::<Memory>(&env, this)?;
        Ok(mem.data_size(&mut *store) as jlong)
    }

    fn new_memory(
        _env: &JNIEnv,
        _clazz: JClass,
        store_ptr: jlong,
        min: jint,
        max: jint,
    ) -> Result<jlong, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let ty = MemoryType::new(Limits::new(
            min as u32,
            if max < 0 { None } else { Some(max as u32) },
        ));
        let mem = Memory::new(&mut *store, ty)?;
        Ok(interop::into_raw::<Memory>(mem))
    }

    fn native_size(env: &JNIEnv, this: JObject, store_ptr: jlong) -> Result<jint, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let mem = interop::get_inner::<Memory>(&env, this)?;
        Ok(mem.size(&mut *store) as jint)
    }

    fn native_grow(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
        delta_pages: jint,
    ) -> Result<jint, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let mem = interop::get_inner::<Memory>(&env, this)?;
        Ok(mem.grow(&mut *store, delta_pages as u32)? as jint)
    }
}
