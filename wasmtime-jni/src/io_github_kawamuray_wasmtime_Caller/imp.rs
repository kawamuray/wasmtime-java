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
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        name: JString<'a>,
    ) -> Result<jobject, Self::Error> {
        let mut caller = interop::get_inner::<Caller<StoreData>>(env, &this)?;
        Ok(match caller.get_export(&utils::get_string(env, &name)?) {
            None => JObject::null().into_raw(),
            Some(ext) => wextern::into_java(env, ext)?.into_raw(),
        })
    }

    fn data(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<jobject, Self::Error> {
        let caller = interop::get_inner::<Caller<StoreData>>(env, &this)?;
        Ok(caller
            .data()
            .java_data
            .as_ref()
            .map(|r| r.as_obj().as_raw())
            .unwrap_or(JObject::null().into_raw()))
    }
}
