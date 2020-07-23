use crate::errors::Result;
use crate::{interop, utils, wrap_error};

use jni::objects::{JClass, JObject, JString};
use jni::sys::{jbyteArray, jlong};
use jni::JNIEnv;
use wasmtime::{Engine, Module};

#[no_mangle]
extern "system" fn Java_wasmtime_Module_newModule(
    env: JNIEnv,
    _clazz: JClass,
    engine_ptr: jlong,
    bytes: jbyteArray,
) -> jlong {
    wrap_error!(env, new_module(&env, engine_ptr, bytes))
}

fn new_module(env: &JNIEnv, engine_ptr: jlong, bytes: jbyteArray) -> Result<jlong> {
    let bytes = env.convert_byte_array(bytes)?;
    let module = Module::new(&*interop::ref_from_raw::<Engine>(engine_ptr)?, &bytes)?;
    Ok(interop::into_raw::<Module>(module))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Module_newFromFile(
    env: JNIEnv,
    _clazz: JClass,
    engine_ptr: jlong,
    file_name: JString,
) -> jlong {
    wrap_error!(env, from_file(&env, engine_ptr, file_name))
}

fn from_file(env: &JNIEnv, engine_ptr: jlong, file_name: JString) -> Result<jlong> {
    let filename = utils::get_string(env, *file_name)?;
    let module = Module::from_file(&*interop::ref_from_raw::<Engine>(engine_ptr)?, &filename)?;
    Ok(interop::into_raw::<Module>(module))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Module_newFromBinary(
    env: JNIEnv,
    _clazz: JClass,
    engine_ptr: jlong,
    bytes: jbyteArray,
) -> jlong {
    wrap_error!(env, from_binary(&env, engine_ptr, bytes))
}

fn from_binary(env: &JNIEnv, engine_ptr: jlong, bytes: jbyteArray) -> Result<jlong> {
    let bytes = env.convert_byte_array(bytes)?;
    let module = Module::from_binary(&*interop::ref_from_raw::<Engine>(engine_ptr)?, &bytes)?;
    Ok(interop::into_raw::<Module>(module))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Module_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, interop::take_inner::<Module>(&env, this));
}
