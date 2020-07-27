use crate::errors::Result;
use crate::{interop, wasi_config, wrap_error};
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::JNIEnv;
use wasmtime::{Linker, Store};
use wasmtime_wasi::Wasi;

fn new_wasi(env: &JNIEnv, store_ptr: jlong, config: JObject) -> Result<jlong> {
    let store = interop::ref_from_raw::<Store>(store_ptr)?;
    let ctx = wasi_config::ctx_from_java(env, config)?;
    let wasi = Wasi::new(&store, ctx);
    Ok(interop::into_raw(wasi))
}

#[no_mangle]
extern "system" fn Java_wasmtime_wasi_Wasi_newWasi(
    env: JNIEnv,
    _clazz: JClass,
    store_ptr: jlong,
    config: JObject,
) -> jlong {
    wrap_error!(env, new_wasi(&env, store_ptr, config))
}

fn add_to_linker(env: &JNIEnv, this: JObject, linker_ptr: jlong) -> Result<()> {
    let wasi = interop::get_inner::<Wasi>(env, this)?;
    let mut linker = interop::ref_from_raw::<Linker>(linker_ptr)?;
    wasi.add_to_linker(&mut linker)?;
    Ok(())
}

#[no_mangle]
extern "system" fn Java_wasmtime_wasi_Wasi_nativeAddToLinker(
    env: JNIEnv,
    this: JObject,
    linker_ptr: jlong,
) {
    wrap_error!(env, add_to_linker(&env, this, linker_ptr))
}

#[no_mangle]
extern "system" fn Java_wasmtime_wasi_Wasi_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, interop::take_inner::<Wasi>(&env, this));
}
