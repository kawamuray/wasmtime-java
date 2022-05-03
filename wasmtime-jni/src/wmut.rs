use crate::errors::{Result};
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime::Mutability;

pub const MUT_TYPE: &str = "io/github/kawamuray/wasmtime/Mutability";

pub fn mutability_into_java<'a>(env: &'a JNIEnv, mutability: Mutability) -> Result<JObject<'a>> {
    match mutability {
        Mutability::Const => mutability_from_enum(env, "CONST"),
        Mutability::Var => mutability_from_enum(env, "VAR")
    }
}

fn mutability_from_enum<'a>(env: &'a JNIEnv, mutability: &str) -> Result<JObject<'a>> {
    const MUT_TYPE_RETURN: &str = "Lio/github/kawamuray/wasmtime/Mutability;";
    Ok(env
        .get_static_field(MUT_TYPE, mutability, MUT_TYPE_RETURN)?
        .l()?)
}
