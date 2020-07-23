use crate::interop;
use crate::wrap_error;
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::{self, JNIEnv};
use wasmtime::{Engine, Store};

#[no_mangle]
extern "system" fn Java_wasmtime_Store_newStore(_env: JNIEnv, _clazz: JClass) -> jlong {
    let store = Store::default();
    interop::into_raw(store)
}

fn engine_ptr(env: &JNIEnv, this: JObject) -> jni::errors::Result<jlong> {
    let store = interop::get_inner::<Store>(&env, this)?;
    let engine: Engine = store.engine().clone();
    Ok(interop::into_raw::<Engine>(engine))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Store_enginePtr(env: JNIEnv, this: JObject) -> jlong {
    wrap_error!(env, engine_ptr(&env, this))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Store_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, interop::take_inner::<Store>(&env, this));
}
