use super::JniInterruptHandle;
use crate::errors::{self, Result};
use crate::interop;
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime::InterruptHandle;

pub(super) struct JniInterruptHandleImpl;

impl<'a> JniInterruptHandle<'a> for JniInterruptHandleImpl {
    type Error = errors::Error;

    fn interrupt(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        let interrupt_handle = interop::get_inner::<InterruptHandle>(&env, this)?;
        interrupt_handle.interrupt();
        Ok(())
    }
}
