use super::JniLinker;
use crate::errors::{self, Result};
use crate::{interop, utils, wextern};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use wasmtime::{Linker, Store};

pub(super) struct JniLinkerImpl;

impl<'a> JniLinker<'a> for JniLinkerImpl {
    type Error = errors::Error;

    fn new_linker(_env: &JNIEnv, _clazz: JClass, store_ptr: jlong) -> Result<jlong, Self::Error> {
        let store = interop::ref_from_raw::<Store>(store_ptr)?;
        let linker = Linker::new(&store);
        Ok(interop::into_raw::<Linker>(linker))
    }

    fn native_module(
        env: &JNIEnv,
        this: JObject,
        module_name: JString,
        module_ptr: jlong,
    ) -> Result<(), Self::Error> {
        let mut linker = interop::get_inner::<Linker>(env, this)?;
        let name = utils::get_string(env, *module_name)?;
        let module = interop::ref_from_raw(module_ptr)?;
        linker.module(&name, &module)?;
        Ok(())
    }

    fn native_define(
        env: &JNIEnv,
        this: JObject,
        module_name: JString,
        name: JString,
        extern_item: JObject,
    ) -> Result<(), Self::Error> {
        let mut linker = interop::get_inner::<Linker>(env, this)?;
        let module_name = utils::get_string(env, *module_name)?;
        let name = utils::get_string(env, *name)?;
        let ext = wextern::from_java(env, extern_item)?;
        linker.define(&module_name, &name, ext)?;
        Ok(())
    }

    fn native_get_one_by_name(
        env: &JNIEnv,
        this: JObject,
        module: JString,
        name: JString,
    ) -> Result<jobject, Self::Error> {
        let linker = interop::get_inner::<Linker>(env, this)?;
        let module = utils::get_string(env, *module)?;
        let name = utils::get_string(env, *name)?;
        let ext = linker.get_one_by_name(&module, &name)?;
        Ok(wextern::into_java(env, ext)?.into_inner())
    }

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::take_inner::<Linker>(&env, this)?;
        Ok(())
    }
}
