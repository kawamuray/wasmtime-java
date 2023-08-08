use super::JniGlobal;
use crate::errors::{self, Result};
use crate::store::StoreData;
use crate::{interop, wval};
use jni::objects::JObject;
use jni::sys::{jlong, jobject};
use jni::JNIEnv;
use wasmtime::{Global, Store};

pub(super) struct JniGlobalImpl;

impl<'a> JniGlobal<'a> for JniGlobalImpl {
    type Error = errors::Error;

    fn native_get(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let global = interop::get_inner::<Global>(env, &this)?;

        let val = global.get(&mut *store);

        Ok(wval::into_java(env, val)?.into_raw())
    }

    fn native_set(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        val: JObject<'a>,
    ) -> Result<(), Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let global = interop::get_inner::<Global>(env, &this)?;
        global.set(&mut *store, wval::from_java(env, val)?)?;
        Ok(())
    }

    fn native_mutable(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<u8, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let global = interop::get_inner::<Global>(env, &this)?;
        let ty = global.ty(&mut *store);
        match ty.mutability() {
            wasmtime::Mutability::Const => Ok(0),
            wasmtime::Mutability::Var => Ok(1),
        }
    }

    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error> {
        interop::dispose_inner::<Global>(env, &this)?;
        Ok(())
    }
}
