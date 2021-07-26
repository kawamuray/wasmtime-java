use super::JniCaller;
use crate::errors;
use crate::interop;
use crate::store::StoreData;
use crate::utils;
use crate::wextern;
use jni::objects::*;
use jni::sys::*;
use jni::{self, JNIEnv};
use wasmtime::Caller;

pub(super) struct JniCallerImpl;

impl<'a> JniCaller<'a> for JniCallerImpl {
    type Error = errors::Error;

    fn native_get_export(
        env: &JNIEnv,
        this: JObject,
        name: JString,
    ) -> Result<jobject, Self::Error> {
        let mut caller = interop::get_inner::<Caller<StoreData>>(env, this)?;
        Ok(
            match caller.get_export(&utils::get_string(env, name.into())?) {
                None => JObject::null().into_inner(),
                Some(ext) => wextern::into_java(env, ext)?.into_inner(),
            },
        )
    }

    fn data(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error> {
        let caller = interop::get_inner::<Caller<StoreData>>(env, this)?;
        Ok(caller
            .data()
            .java_data
            .as_ref()
            .map(|r| r.as_obj().into_inner())
            .unwrap_or(JObject::null().into_inner()))
    }
}
