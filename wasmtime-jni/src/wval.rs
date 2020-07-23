use crate::errors::{Error, Result};
use crate::utils;
use jni::objects::JObject;
use jni::sys::jobjectArray;
use jni::JNIEnv;
use wasmtime::{Val, ValType};

pub fn from_java(env: &JNIEnv, obj: JObject) -> Result<Val> {
    let ty = env.get_field(obj, "type", "Lwasmtime/Val$Type;")?.l()?;
    let name = utils::enum_name(&env, ty)?;
    Ok(match name.as_str() {
        "I32" => {
            let val = env.call_method(obj, "i32", "()I", &[])?.i()?;
            Val::from(val)
        }
        "I64" => {
            let val = env.call_method(obj, "i64", "()J", &[])?.j()?;
            Val::from(val)
        }
        "F32" => {
            let val = env.call_method(obj, "f32", "()F", &[])?.f()?;
            Val::from(val)
        }
        "F64" => {
            let val = env.call_method(obj, "f64", "()D", &[])?.d()?;
            Val::from(val)
        }
        _ => return Err(Error::UnknownEnum(name)),
    })
}

pub fn into_java<'a>(env: &'a JNIEnv, val: Val) -> Result<JObject<'a>> {
    Ok(match val {
        Val::I32(v) => env
            .call_static_method("wasmtime/Val", "fromI32", "(I)Lwasmtime/Val;", &[v.into()])?
            .l()?,
        Val::I64(v) => env
            .call_static_method("wasmtime/Val", "fromI64", "(J)Lwasmtime/Val;", &[v.into()])?
            .l()?,
        Val::F32(v) => env
            .call_static_method(
                "wasmtime/Val",
                "fromF32",
                "(F)Lwasmtime/Val;",
                &[f32::from_bits(v).into()],
            )?
            .l()?,
        Val::F64(v) => env
            .call_static_method(
                "wasmtime/Val",
                "fromF64",
                "(D)Lwasmtime/Val;",
                &[f64::from_bits(v).into()],
            )?
            .l()?,
        _ => return Err(Error::NotImplemented),
    })
}

pub fn type_from_java(env: &JNIEnv, obj: JObject) -> Result<ValType> {
    let name = utils::enum_name(&env, obj)?;
    Ok(match name.as_str() {
        "I32" => ValType::I32,
        "I64" => ValType::I64,
        "F32" => ValType::F32,
        "F64" => ValType::F64,
        _ => return Err(Error::UnknownEnum(name)),
    })
}

pub fn types_from_java(env: &JNIEnv, array: jobjectArray) -> Result<Vec<ValType>> {
    let iter = utils::JavaArrayIter::new(env, array)?;
    let mut ret = Vec::with_capacity(iter.len());
    for obj in iter {
        let ty = type_from_java(env, obj?)?;
        ret.push(ty);
    }
    Ok(ret)
}
