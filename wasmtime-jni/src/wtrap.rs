use crate::errors::{Error, Result};
use crate::utils;
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime::Trap;

pub fn from_java(env: &JNIEnv, obj: JObject) -> Result<Trap> {
    let ty = env
        .get_field(obj, "type", "Lio/github/kawamuray/wasmtime/Trap$Type;")?
        .l()?;
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

pub fn into_java<'a>(env: &JNIEnv<'a>, trap: &Trap) -> jni::errors::Result<JObject<'a>> {
    if let Some(status) = trap.i32_exit_status() {
        Ok(env
            .call_static_method(
                "io/github/kawamuray/wasmtime/Trap",
                "fromExitCode",
                "(I)Lio/github/kawamuray/wasmtime/Trap;",
                &[status.into()],
            )?
            .l()?)
    } else {
        let jmsg = env.new_string(trap.to_string())?;
        Ok(env
            .call_static_method(
                "io/github/kawamuray/wasmtime/Trap",
                "fromMessage",
                "(Ljava/lang/String;)Lio/github/kawamuray/wasmtime/Trap;",
                &[jmsg.into()],
            )?
            .l()?)
    }
}
