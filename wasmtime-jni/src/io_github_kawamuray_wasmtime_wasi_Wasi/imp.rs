use super::JniWasi;
use crate::errors::{self, Result};
use crate::{interop, wasi_config};
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::JNIEnv;
use wasmtime::{Linker, Store};
use wasmtime_wasi::Wasi;

pub(super) struct JniWasiImpl;

impl<'a> JniWasi<'a> for JniWasiImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::take_inner::<Wasi>(&env, this)?;
        Ok(())
    }

    fn new_wasi(
        env: &JNIEnv,
        _clazz: JClass,
        store_ptr: jlong,
        config: JObject,
    ) -> Result<jlong, Self::Error> {
        let store = interop::ref_from_raw::<Store>(store_ptr)?;
        let ctx = wasi_config::ctx_from_java(env, config)?;
        let wasi = Wasi::new(&store, ctx);
        Ok(interop::into_raw(wasi))
    }

    fn native_add_to_linker(
        env: &JNIEnv,
        this: JObject,
        linker_ptr: jlong,
    ) -> Result<(), Self::Error> {
        let wasi = interop::get_inner::<Wasi>(env, this)?;
        let mut linker = interop::ref_from_raw::<Linker>(linker_ptr)?;
        wasi.add_to_linker(&mut linker)?;
        Ok(())
    }
}
