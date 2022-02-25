use super::JniLinker;
use crate::errors::{self, Result};
use crate::store::StoreData;
use crate::{interop, utils, wextern};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jlong, jobject, jobjectArray, jsize};
use jni::JNIEnv;
use std::collections::HashSet;
use wasmtime::{Engine, Linker, Store};

pub(super) struct JniLinkerImpl;

impl<'a> JniLinker<'a> for JniLinkerImpl {
    type Error = errors::Error;

    fn new_linker(_env: &JNIEnv, _clazz: JClass, engine_ptr: jlong) -> Result<jlong, Self::Error> {
        let engine = interop::ref_from_raw::<Engine>(engine_ptr)?;
        let linker = Linker::new(&engine);
        Ok(interop::into_raw::<Linker<StoreData>>(linker))
    }

    fn native_module(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
        module_name: JString,
        module_ptr: jlong,
    ) -> Result<(), Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let mut linker = interop::get_inner::<Linker<StoreData>>(env, this)?;
        let name = utils::get_string(env, *module_name)?;
        let module = interop::ref_from_raw(module_ptr)?;
        linker.module(&mut *store, &name, &module)?;
        Ok(())
    }

    fn native_define(
        env: &JNIEnv,
        this: JObject,
        module_name: JString,
        name: JString,
        extern_item: JObject,
    ) -> Result<(), Self::Error> {
        let mut linker = interop::get_inner::<Linker<StoreData>>(env, this)?;
        let module_name = utils::get_string(env, *module_name)?;
        let name = utils::get_string(env, *name)?;
        let ext = wextern::from_java(env, extern_item)?;
        linker.define(&module_name, &name, ext)?;
        Ok(())
    }

    fn native_get(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
        module: JString,
        name: JString,
    ) -> Result<jobject, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let linker = interop::get_inner::<Linker<StoreData>>(env, this)?;
        let module = utils::get_string(env, *module)?;
        let name = utils::get_string(env, *name)?;
        let ret = match linker.get(&mut *store, &module, Some(&name)) {
            Some(ext) => wextern::into_java(env, ext)?.into_inner(),
            None => JObject::null().into_inner(),
        };
        Ok(ret)
    }

    fn native_externs(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
        module: JString,
    ) -> Result<jobjectArray, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let linker = interop::get_inner::<Linker<StoreData>>(env, this)?;
        let module = utils::get_string(env, *module)?;
        let mut vec: Vec<String> = Vec::new();
        for item in linker.iter(&mut *store) {
            if item.0.eq(&module) {
                vec.push(item.1.to_string())
            }
        }
        let ret = env.new_object_array(vec.len() as i32, "java/lang/Object", JObject::null())?;
        for (i, item) in vec.iter().enumerate() {
            let value: JString = env.new_string(item).unwrap().into();
            env.set_object_array_element(ret, i as jsize, JObject::from(value))?;
        }
        Ok(ret.into())
    }

    fn native_modules(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
    ) -> Result<jobjectArray, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let linker = interop::get_inner::<Linker<StoreData>>(env, this)?;
        let mut names: HashSet<String> = HashSet::new();
        for item in linker.iter(&mut *store) {
            names.insert(item.0.to_string());
        }
        let ret = env.new_object_array(names.len() as i32, "java/lang/Object", JObject::null())?;
        for (i, item) in names.iter().enumerate() {
            let value: JString = env.new_string(item).unwrap().into();
            env.set_object_array_element(ret, i as jsize, JObject::from(value))?;
        }
        Ok(ret.into())
    }

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::dispose_inner::<Linker<StoreData>>(&env, this)?;
        Ok(())
    }
}
