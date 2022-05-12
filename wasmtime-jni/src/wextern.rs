use crate::errors::{Error, Result};
use crate::{interop, utils, wval};
use jni::objects::JObject;
use jni::JNIEnv;
use jni::sys::jint;
use wasmtime::{Extern, ExternType, Func, Global, Limits, Memory, Table};
use crate::wmut::{MUT_TYPE, mutability_into_java};
use crate::wval::VAL_TYPE;

pub const EXTERN_TYPE_CLASS: &'static str = "io/github/kawamuray/wasmtime/ExternType";
const LIMIT_TYPE: &str = "io/github/kawamuray/wasmtime/MemoryType$Limit";

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
                &[interop::into_raw::<Func>(func).into()],
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
                &[interop::into_raw::<Memory>(memory).into()],
            )?;
            env.call_static_method(
                "io/github/kawamuray/wasmtime/Extern",
                "fromMemory",
                "(Lio/github/kawamuray/wasmtime/Memory;)Lio/github/kawamuray/wasmtime/Extern;",
                &[mem_obj.into()],
            )?
            .l()?
        }
        Extern::Table(table) => {
            let table_obj = env.new_object(
                "io/github/kawamuray/wasmtime/Table",
                "(J)V",
                &[interop::into_raw::<Table>(table).into()],
            )?;
            env.call_static_method(
                "io/github/kawamuray/wasmtime/Extern",
                "fromTable",
                "(Lio/github/kawamuray/wasmtime/Table;)Lio/github/kawamuray/wasmtime/Extern;",
                &[table_obj.into()],
            )?
            .l()?
        }
        Extern::Global(global) => {
            let global_obj = env.new_object(
                "io/github/kawamuray/wasmtime/Global",
                "(J)V",
                &[interop::into_raw::<Global>(global).into()],
            )?;
            env.call_static_method(
                "io/github/kawamuray/wasmtime/Extern",
                "fromGlobal",
                "(Lio/github/kawamuray/wasmtime/Global;)Lio/github/kawamuray/wasmtime/Extern;",
                &[global_obj.into()],
            )?
            .l()?
        }
        _ => return Err(Error::NotImplemented),
    })
}

pub fn type_into_java<'a>(env: &'a JNIEnv, ty: ExternType) -> Result<(JObject<'a>, JObject<'a>)> {
    let (ty, ty_obj) = match ty {
        ExternType::Func(func) => {
            let results = wval::types_into_java_array(env, func.results());
            let params = wval::types_into_java_array(env, func.params());

            (
                into_java_extern_type(env, "FUNC"),
                env.new_object(
                    "io/github/kawamuray/wasmtime/FuncType",
                    format!("([L{};[L{};)V", VAL_TYPE, VAL_TYPE),
                    &[params?.into(), results?.into()],
                ),
            )
        }
        ExternType::Global(global) => (
            into_java_extern_type(env, "GLOBAL"),
            env.new_object(
                "io/github/kawamuray/wasmtime/GlobalType",
                format!("(L{};L{};)V", VAL_TYPE, MUT_TYPE),
                &[
                    wval::type_into_java(env, global.content().to_owned())?
                        .into_inner()
                        .into(),
                    mutability_into_java(env, global.mutability())?
                        .into_inner()
                        .into(),
                ],
            ),
        ),
        ExternType::Table(tab) => {
            const TABLE_TYPE: &str = "io/github/kawamuray/wasmtime/TableType";
            let limit = limit_into_java(env, tab.limits());
            let val = wval::type_into_java(env, tab.element().to_owned());
            let table = env.new_object(
                TABLE_TYPE,
                format!("(L{};L{};)V", VAL_TYPE, LIMIT_TYPE),
                &[val?.into_inner().into(), limit?.into_inner().into()],
            );

            (into_java_extern_type(env, "TABLE"), table)
        }
        ExternType::Memory(mem) => {
            const MEMORY_TYPE: &str = "io/github/kawamuray/wasmtime/MemoryType";
            let limit = limit_into_java(env, mem.limits());
            let mem = env.new_object(
                MEMORY_TYPE,
                format!("(L{};)V", LIMIT_TYPE),
                &[limit?.into_inner().into()],
            );

            (into_java_extern_type(env, "MEMORY"), mem)
        }
        // WebAssembly module-linking proposal
        ExternType::Instance(_) => (into_java_extern_type(env, "INSTANCE"), Ok(JObject::null())),
        // WebAssembly module-linking proposal
        ExternType::Module(_) => (into_java_extern_type(env, "MODULE"), Ok(JObject::null())),
    };

    Ok((ty?, ty_obj?))
}

pub fn unknown<'a>(env: &'a JNIEnv) -> Result<JObject<'a>> {
    Ok(env
        .get_static_field(
            "io/github/kawamuray/wasmtime/Extern",
            "UNKNOWN",
            "Lio/github/kawamuray/wasmtime/Extern;",
        )?
        .l()?)
}

fn limit_into_java<'a>(env: &'a JNIEnv, limits: &Limits) -> jni::errors::Result<JObject<'a>> {
    let min = limits.min() as jint;
    let max = match limits.max() {
        None => -1,
        Some(max) => max as jint,
    };
    env.new_object(LIMIT_TYPE, "(II)V", &[min.into(), max.into()])
}

fn into_java_extern_type<'a>(env: &'a JNIEnv, ty: &'a str) -> jni::errors::Result<JObject<'a>> {
    env.get_static_field(EXTERN_TYPE_CLASS, ty, format!("L{};", EXTERN_TYPE_CLASS))?
        .l()
}
