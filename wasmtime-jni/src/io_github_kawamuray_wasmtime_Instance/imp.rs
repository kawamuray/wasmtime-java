use super::JniInstance;
use crate::errors::{self, Result};
use crate::store::StoreData;
use crate::{interop, utils, wextern};
use jni::objects::{JClass, JObject, JString};
use jni::sys::jlong;
use jni::JNIEnv;
use wasmtime::{Func, Instance, Memory, Module, Store};

pub(super) struct JniInstanceImpl;

impl<'a> JniInstance<'a> for JniInstanceImpl {
    type Error = errors::Error;

    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error> {
        interop::dispose_inner::<Instance>(env, &this)?;
        Ok(())
    }

    fn native_get_func(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        name: JString<'a>,
    ) -> Result<jlong, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let instance = interop::get_inner::<Instance>(env, &this)?;
        let name = utils::get_string(env, &name)?;
        let func = instance.get_func(&mut *store, &name);
        Ok(func.map(|f| interop::into_raw::<Func>(f)).unwrap_or(0))
    }

    fn native_get_memory(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        name: JString<'a>,
    ) -> Result<jlong, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let instance = interop::get_inner::<Instance>(env, &this)?;
        let name = utils::get_string(env, &name)?;
        let memory = instance.get_memory(&mut *store, &name);
        Ok(memory.map(|m| interop::into_raw::<Memory>(m)).unwrap_or(0))
    }

    fn new_instance(
        env: &mut JNIEnv<'a>,
        _clazz: JClass<'a>,
        store_ptr: jlong,
        module_ptr: jlong,
        externs: jni::sys::jarray,
    ) -> Result<jlong, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let module = interop::ref_from_raw::<Module>(module_ptr)?;

        // Convert Java Extern into wasmtime Extern
        let mut iter = utils::JavaArrayIter::new(env, externs)?;
        let mut imports = Vec::with_capacity(iter.len());
        while let Some(ext) = iter.next(env) {
            imports.push(wextern::from_java(env, ext?)?);
        }

        let instance = Instance::new(&mut *store, &module, &imports)?;
        Ok(interop::into_raw::<Instance>(instance))
    }
}
