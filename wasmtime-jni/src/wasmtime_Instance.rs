use crate::errors::Result;
use crate::{interop, utils, wextern, wrap_error};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jlong, jobjectArray};
use jni::JNIEnv;
use wasmtime::{Instance, Module, Store};

#[no_mangle]
extern "system" fn Java_wasmtime_Instance_newInstance(
    env: JNIEnv,
    _clazz: JClass,
    store_ptr: jlong,
    module_ptr: jlong,
    externs: jobjectArray,
) -> jlong {
    wrap_error!(env, new_instance(&env, store_ptr, module_ptr, externs))
}

fn new_instance(
    env: &JNIEnv,
    store_ptr: jlong,
    module_ptr: jlong,
    externs: jobjectArray,
) -> Result<jlong> {
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

#[no_mangle]
extern "system" fn Java_wasmtime_Instance_nativeGetFunc(
    env: JNIEnv,
    this: JObject,
    name: JString,
) -> jlong {
    wrap_error!(env, get_func(&env, this, name))
}

fn get_func(env: &JNIEnv, this: JObject, name: JString) -> Result<jlong> {
    let instance = interop::get_inner::<Instance>(env, this)?;
    let name = utils::get_string(env, *name)?;
    let func = instance.get_func(&name);
    Ok(func.map(interop::into_raw).unwrap_or(0))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Instance_nativeGetMemory(
    env: JNIEnv,
    this: JObject,
    name: JString,
) -> jlong {
    wrap_error!(env, get_memory(&env, this, name))
}

fn get_memory(env: &JNIEnv, this: JObject, name: JString) -> Result<jlong> {
    let instance = interop::get_inner::<Instance>(env, this)?;
    let name = utils::get_string(env, *name)?;
    let memory = instance.get_memory(&name);
    Ok(memory.map(interop::into_raw).unwrap_or(0))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Instance_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, interop::take_inner::<Instance>(&env, this));
}
