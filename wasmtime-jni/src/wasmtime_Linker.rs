use crate::errors::Result;
use crate::{interop, utils, wextern, wrap_error};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use wasmtime::{Linker, Store};

fn new_linker(store_ptr: jlong) -> Result<jlong> {
    let store = interop::ref_from_raw::<Store>(store_ptr)?;
    let linker = Linker::new(&store);
    Ok(interop::into_raw::<Linker>(linker))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_newLinker(
    env: JNIEnv,
    _clazz: JClass,
    store_ptr: jlong,
) -> jlong {
    wrap_error!(env, new_linker(store_ptr))
}

fn add_module(env: &JNIEnv, this: JObject, module_name: JString, module_ptr: jlong) -> Result<()> {
    let mut linker = interop::get_inner::<Linker>(env, this)?;
    let name = utils::get_string(env, *module_name)?;
    let module = interop::ref_from_raw(module_ptr)?;
    linker.module(&name, &module)?;
    Ok(())
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_nativeModule(
    env: JNIEnv,
    this: JObject,
    module_name: JString,
    module_ptr: jlong,
) {
    wrap_error!(env, add_module(&env, this, module_name, module_ptr))
}

fn define(
    env: &JNIEnv,
    this: JObject,
    module_name: JString,
    name: JString,
    extn: JObject,
) -> Result<()> {
    let mut linker = interop::get_inner::<Linker>(env, this)?;
    let module_name = utils::get_string(env, *module_name)?;
    let name = utils::get_string(env, *name)?;
    let ext = wextern::from_java(env, extn)?;
    linker.define(&module_name, &name, ext)?;
    Ok(())
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_nativeDefine(
    env: JNIEnv,
    this: JObject,
    module_name: JString,
    name: JString,
    extn: JObject,
) {
    wrap_error!(env, define(&env, this, module_name, name, extn))
}

fn get_one_by_name(env: &JNIEnv, this: JObject, module: JString, name: JString) -> Result<jobject> {
    let linker = interop::get_inner::<Linker>(env, this)?;
    let module = utils::get_string(env, *module)?;
    let name = utils::get_string(env, *name)?;
    let ext = linker.get_one_by_name(&module, &name)?;
    Ok(wextern::into_java(env, ext)?.into_inner())
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_nativeGetOneByName(
    env: JNIEnv,
    this: JObject,
    module: JString,
    name: JString,
) -> jobject {
    wrap_error!(
        env,
        get_one_by_name(&env, this, module, name),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, interop::take_inner::<Linker>(&env, this));
}
