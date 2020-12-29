use crate::errors::{Error, Result};
use crate::{interop, utils};
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime::{Extern, Func, Memory};

pub fn from_java(env: &JNIEnv, obj: JObject) -> Result<Extern> {
    let ty = env
        .get_field(obj, "type", "Lio/github/kawamuray/wasmtime/Extern$Type;")?
        .l()?;
    let name = utils::enum_name(&env, ty)?;
    let extn = match name.as_str() {
        "FUNC" => {
            let fn_obj = env
                .call_method(obj, "func", "()Lio/github/kawamuray/wasmtime/Func;", &[])?
                .l()?;
            let func = interop::get_inner::<Func>(&env, fn_obj)?;
            Extern::from(func.clone())
        }
        "MEMORY" => {
            let mem_obj = env
                .call_method(
                    obj,
                    "memory",
                    "()Lio/github/kawamuray/wasmtime/Memory;",
                    &[],
                )?
                .l()?;
            let memory = interop::get_inner::<Memory>(&env, mem_obj)?;
            Extern::from(memory.clone())
        }
        _ => return Err(Error::UnknownEnum(name)),
    };
    Ok(extn)
}

pub fn into_java<'a>(env: &'a JNIEnv, ext: Extern) -> Result<JObject<'a>> {
    Ok(match ext {
        Extern::Func(func) => {
            let fn_obj = env.new_object(
                "io/github/kawamuray/wasmtime/Func",
                "(J)V",
                &[interop::into_raw(func).into()],
            )?;
            env.call_static_method(
                "io/github/kawamuray/wasmtime/Extern",
                "fromFunc",
                "(Lio/github/kawamuray/wasmtime/Func;)Lio/github/kawamuray/wasmtime/Extern;",
                &[fn_obj.into()],
            )?
            .l()?
        }
        Extern::Memory(memory) => {
            let mem_obj = env.new_object(
                "io/github/kawamuray/wasmtime/Memory",
                "(J)V",
                &[interop::into_raw(memory).into()],
            )?;
            env.call_static_method(
                "io/github/kawamuray/wasmtime/Extern",
                "fromMemory",
                "(Lio/github/kawamuray/wasmtime/Memory;)Lio/github/kawamuray/wasmtime/Extern;",
                &[mem_obj.into()],
            )?
            .l()?
        }
        _ => return Err(Error::NotImplemented),
    })
}
