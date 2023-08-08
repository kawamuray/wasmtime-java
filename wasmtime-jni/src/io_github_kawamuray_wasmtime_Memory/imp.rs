use super::JniMemory;
use crate::errors::{self, Result};
use crate::interop;
use crate::store::StoreData;
use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jint, jlong, jobject, JNI_TRUE};
use jni::JNIEnv;
use wasmtime::{Memory, MemoryType, Store};

pub(super) struct JniMemoryImpl;

impl<'a> JniMemory<'a> for JniMemoryImpl {
    type Error = errors::Error;

    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error> {
        interop::dispose_inner::<Memory>(env, &this)?;
        Ok(())
    }

    fn native_buffer(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let mem = interop::get_inner::<Memory>(env, &this)?;
        let ptr = mem.data_mut(&mut *store);
        Ok(unsafe { env.new_direct_byte_buffer(ptr.as_mut_ptr(), ptr.len()) }?.into_raw())
    }

    fn native_data_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jlong, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let mem = interop::get_inner::<Memory>(env, &this)?;
        Ok(mem.data_size(&mut *store) as jlong)
    }

    fn new_memory(
        _env: &mut JNIEnv<'a>,
        _clazz: JClass,
        store_ptr: jlong,
        min: jlong,
        max: jlong,
        is_64: jboolean,
    ) -> Result<jlong, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let max = if max < 0 { None } else { Some(max as u64) };
        let ty = if is_64 == JNI_TRUE {
            MemoryType::new64(min as u64, max)
        } else {
            MemoryType::new(min as u32, max.map(|v| v as u32))
        };
        let mem = Memory::new(&mut *store, ty)?;
        Ok(interop::into_raw::<Memory>(mem))
    }

    fn native_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jint, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let mem = interop::get_inner::<Memory>(env, &this)?;
        Ok(mem.size(&mut *store) as jint)
    }

    fn native_grow(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        delta_pages: jlong,
    ) -> Result<jint, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let mem = interop::get_inner::<Memory>(env, &this)?;
        Ok(mem.grow(&mut *store, delta_pages as u64)? as jint)
    }
}
