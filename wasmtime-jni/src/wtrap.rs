use crate::errors::{Error, Result};
use crate::utils;
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime::Trap;

pub fn from_java(env: &JNIEnv, obj: JObject) -> Result<Trap> {
    let ty = env.get_field(obj, "type", "Lwasmtime/Trap$Type;")?.l()?;
    let name = utils::enum_name(&env, ty)?;
    Ok(match name.as_str() {
        "MESSAGE" => {
            let message = utils::get_string_field(env, obj, "message")?;
            Trap::new(message)
        }
        "I32_EXIT" => {
            let status_code = env.get_field(obj, "", "I")?.i()?;
            Trap::i32_exit(status_code)
        }
        _ => return Err(Error::UnknownEnum(name)),
    })
}
