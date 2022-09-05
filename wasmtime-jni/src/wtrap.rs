use crate::errors::{Error, Result};
use crate::utils;
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime::{Trap, TrapCode};

pub fn from_java(env: &JNIEnv, obj: JObject) -> Result<Trap> {
    let ty = env
        .get_field(obj, "reason", "Lio/github/kawamuray/wasmtime/Trap$Reason;")?
        .l()?;
    let name = utils::enum_name(&env, ty)?;
    Ok(match name.as_str() {
        "MESSAGE" => {
            let message = utils::get_string_field(env, obj, "message")?;
            Trap::new(message)
        }
        "I32_EXIT" => {
            let status_code = env.get_field(obj, "exitCode", "I")?.i()?;
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
    } else if let Some(code) = trap.trap_code() {
        let jtrapcode = trapcode_into_java(env, code)?;
        Ok(env
            .call_static_method(
                "io/github/kawamuray/wasmtime/Trap",
                "fromTrapCode",
                "(Lio/github/kawamuray/wasmtime/Trap$TrapCode;)Lio/github/kawamuray/wasmtime/Trap;",
                &[jtrapcode.into()],
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

fn trapcode_into_java<'a>(env: &'a JNIEnv, code: TrapCode) -> jni::errors::Result<JObject<'a>> {
    let name = match code {
        TrapCode::StackOverflow => "STACK_OVERFLOW",
        TrapCode::MemoryOutOfBounds => "MEMORY_OUT_OF_BOUNDS",
        TrapCode::HeapMisaligned => "HEAP_MISALIGNED",
        TrapCode::TableOutOfBounds => "TABLE_OUT_OF_BOUNDS",
        TrapCode::IndirectCallToNull => "INDIRECT_CALL_TO_NULL",
        TrapCode::BadSignature => "BAD_SIGNATURE",
        TrapCode::IntegerOverflow => "INTEGER_OVERFLOW",
        TrapCode::IntegerDivisionByZero => "INTEGER_DIVISION_BY_ZERO",
        TrapCode::BadConversionToInteger => "BAD_CONVERSION_TO_INTEGER",
        TrapCode::UnreachableCodeReached => "UNREACHABLE_CODE_REACHED",
        TrapCode::Interrupt => "INTERRUPT",
        TrapCode::AlwaysTrapAdapter => "ALWAYS_TRAP_ADAPTER",
        _ => "UNKNOWN",
    };

    env.get_static_field(
        "io/github/kawamuray/wasmtime/Trap$TrapCode",
        name,
        "Lio/github/kawamuray/wasmtime/Trap$TrapCode;",
    )?
    .l()
}
