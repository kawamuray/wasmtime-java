use crate::errors::{Error, Result};
use crate::{interop, utils};
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime::{Extern, Func};

pub fn from_java(env: &JNIEnv, obj: JObject) -> Result<Extern> {
    let ty = env.get_field(obj, "type", "Lwasmtime/Extern$Type;")?.l()?;
    let name = utils::enum_name(&env, ty)?;
    let extn = match name.as_str() {
        "FUNC" => {
            let fn_obj = env
                .call_method(obj, "func", "()Lwasmtime/Func;", &[])?
                .l()?;
            let func = interop::get_inner::<Func>(&env, fn_obj)?;
            Extern::from(func.clone())
        }
        _ => return Err(Error::UnknownEnum(name)),
    };
    Ok(extn)
}

pub fn into_java<'a>(env: &'a JNIEnv, ext: Extern) -> Result<JObject<'a>> {
    Ok(match ext {
        Extern::Func(func) => {
            let fn_obj = env.new_object(
                "wasmtime/Func",
                "(J)Lwasmtime/Func;",
                &[interop::into_raw(func).into()],
            )?;
            env.call_static_method(
                "wasmtime/Extern",
                "fromFunc",
                "(Lwasmtime/Func;)Lwasmtime/Extern;",
                &[fn_obj.into()],
            )?
            .l()?
        }
        _ => return Err(Error::NotImplemented),
    })
}
