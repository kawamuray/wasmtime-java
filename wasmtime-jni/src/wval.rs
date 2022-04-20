use crate::errors::{Error, Result};
use crate::{utils};
use jni::objects::JObject;
use jni::sys::jobjectArray;
use jni::JNIEnv;
use wasmtime::{Val, ValType};
use crate::utils::into_java_array;

pub const VAL_TYPE: &str = "io/github/kawamuray/wasmtime/Val$Type";

pub fn from_java(env: &JNIEnv, obj: JObject) -> Result<Val> {
    let ty = env
        .get_field(obj, "type", "Lio/github/kawamuray/wasmtime/Val$Type;")?
        .l()?;
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
            .call_static_method(
                "io/github/kawamuray/wasmtime/Val",
                "fromI32",
                "(I)Lio/github/kawamuray/wasmtime/Val;",
                &[v.into()],
            )?
            .l()?,
        Val::I64(v) => env
            .call_static_method(
                "io/github/kawamuray/wasmtime/Val",
                "fromI64",
                "(J)Lio/github/kawamuray/wasmtime/Val;",
                &[v.into()],
            )?
            .l()?,
        Val::F32(v) => env
            .call_static_method(
                "io/github/kawamuray/wasmtime/Val",
                "fromF32",
                "(F)Lio/github/kawamuray/wasmtime/Val;",
                &[f32::from_bits(v).into()],
            )?
            .l()?,
        Val::F64(v) => env
            .call_static_method(
                "io/github/kawamuray/wasmtime/Val",
                "fromF64",
                "(D)Lio/github/kawamuray/wasmtime/Val;",
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

fn type_from_enum<'a>(env: &'a JNIEnv, ty: &'a str) -> Result<JObject<'a>> {
    Ok(env
        .get_static_field(VAL_TYPE, ty, "Lio/github/kawamuray/wasmtime/Val$Type;")?
        .l()?)
}

pub fn type_into_java_array<T>(env: &JNIEnv, it: T) -> Result<jobjectArray>
    where
        T: ExactSizeIterator<Item=ValType> + Iterator<Item=ValType>
{
    let mut vec = Vec::with_capacity(it.len());
    for (_, result) in it.enumerate() {
        let x = self::type_into_java(env, result);
        vec.push(x?)
    }
    into_java_array(env, VAL_TYPE, vec)
}

pub fn type_into_java<'a>(env: &'a JNIEnv, val: ValType) -> Result<JObject<'a>> {
    match val {
        ValType::I32 => type_from_enum(env, "I32"),
        ValType::I64 => type_from_enum(env, "I64"),
        ValType::F32 => type_from_enum(env, "F32"),
        ValType::F64 => type_from_enum(env, "F64"),
        ValType::V128 => type_from_enum(env, "V128"),
        ValType::ExternRef => type_from_enum(env, "EXTERN_REF"),
        ValType::FuncRef => type_from_enum(env, "FUNC_REF"),
    }
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
