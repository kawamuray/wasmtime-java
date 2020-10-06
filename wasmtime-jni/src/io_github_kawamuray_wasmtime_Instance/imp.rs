use super::JniInstance;
use crate::errors::{self, Result};
use crate::{interop, utils, wextern};
use jni::objects::{JClass, JObject, JString};
use jni::sys::jlong;
use jni::JNIEnv;
use wasmtime::{Instance, Module, Store};

pub(super) struct JniInstanceImpl;

impl<'a> JniInstance<'a> for JniInstanceImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::take_inner::<Instance>(&env, this)?;
        Ok(())
    }

    fn native_get_func(env: &JNIEnv, this: JObject, name: JString) -> Result<jlong, Self::Error> {
        let instance = interop::get_inner::<Instance>(env, this)?;
        let name = utils::get_string(env, *name)?;
        let func = instance.get_func(&name);
        Ok(func.map(interop::into_raw).unwrap_or(0))
    }

    fn native_get_memory(env: &JNIEnv, this: JObject, name: JString) -> Result<jlong, Self::Error> {
        let instance = interop::get_inner::<Instance>(env, this)?;
        let name = utils::get_string(env, *name)?;
        let memory = instance.get_memory(&name);
        Ok(memory.map(interop::into_raw).unwrap_or(0))
    }

    fn new_instance(
        env: &JNIEnv,
        _clazz: JClass,
        store_ptr: jlong,
        module_ptr: jlong,
        externs: jni::sys::jarray,
    ) -> Result<jlong, Self::Error> {
        let store = interop::ref_from_raw::<Store>(store_ptr)?;
        let module = interop::ref_from_raw::<Module>(module_ptr)?;

        // Convert Java Extern into wasmtime Extern
        let iter = utils::JavaArrayIter::new(env, externs)?;
        let mut imports = Vec::with_capacity(iter.len());
        for ext in iter {
            imports.push(wextern::from_java(env, ext?)?);
        }

        let instance = Instance::new(&store, &module, &imports)?;
        Ok(interop::into_raw::<Instance>(instance))
    }
}
