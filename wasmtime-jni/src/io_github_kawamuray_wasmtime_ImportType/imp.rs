use super::JniImportType;
use crate::types::Import;
use crate::{errors, interop};
use jni::objects::JObject;
use jni::sys::jstring;
use jni::JNIEnv;

pub(super) struct JniImportTypeImpl;

impl<'a> JniImportType<'a> for JniImportTypeImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::dispose_inner::<Import>(env, this)?;
        Ok(())
    }

    fn module(env: &JNIEnv, this: JObject) -> Result<jstring, Self::Error> {
        let module = interop::get_inner::<Import>(env, this)?;
        let m = env.new_string(&module.module).unwrap();
        Ok(m.into_inner().into())
    }

    fn name(env: &JNIEnv, this: JObject) -> Result<jstring, Self::Error> {
        let module = interop::get_inner::<Import>(env, this)?;
        let m = env.new_string(&module.name).unwrap();
        Ok(m.into_inner().into())
    }

    fn ty(env: &JNIEnv, this: JObject) -> Result<jstring, Self::Error> {
        let module = interop::get_inner::<Import>(env, this)?;
        let m = env.new_string(&module.ty).unwrap();
        Ok(m.into_inner().into())
    }
}
