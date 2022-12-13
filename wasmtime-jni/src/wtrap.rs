use crate::errors::{Error, Result};
use crate::utils;
use jni::objects::JObject;
use jni::JNIEnv;
use wasmtime::Trap;

pub fn from_java(env: &JNIEnv, trap: JObject) -> Result<Trap> {
    Ok(match utils::enum_name(env, trap)?.as_str() {
        "STACK_OVERFLOW" => Trap::StackOverflow,
        "MEMORY_OUT_OF_BOUNDS" => Trap::MemoryOutOfBounds,
        "HEAP_MISALIGNED" => Trap::HeapMisaligned,
        "TABLE_OUT_OF_BOUNDS" => Trap::TableOutOfBounds,
        "INDIRECT_CALL_TO_NULL" => Trap::IndirectCallToNull,
        "BAD_SIGNATURE" => Trap::BadSignature,
        "INTEGER_OVERFLOW" => Trap::IntegerOverflow,
        "INTEGER_DIVISION_BY_ZERO" => Trap::IntegerDivisionByZero,
        "BAD_CONVERSION_TO_INTEGER" => Trap::BadConversionToInteger,
        "UNREACHABLE_CODE_REACHED" => Trap::UnreachableCodeReached,
        "INTERRUPT" => Trap::Interrupt,
        "ALWAYS_TRAP_ADAPTER" => Trap::AlwaysTrapAdapter,
        "OUT_OF_FUEL" => Trap::OutOfFuel,
        _ => return Err(Error::NotImplemented),
    })
}

pub fn into_java<'a>(env: &'a JNIEnv, trap: &Trap) -> jni::errors::Result<JObject<'a>> {
    let name = match trap {
        Trap::StackOverflow => "STACK_OVERFLOW",
        Trap::MemoryOutOfBounds => "MEMORY_OUT_OF_BOUNDS",
        Trap::HeapMisaligned => "HEAP_MISALIGNED",
        Trap::TableOutOfBounds => "TABLE_OUT_OF_BOUNDS",
        Trap::IndirectCallToNull => "INDIRECT_CALL_TO_NULL",
        Trap::BadSignature => "BAD_SIGNATURE",
        Trap::IntegerOverflow => "INTEGER_OVERFLOW",
        Trap::IntegerDivisionByZero => "INTEGER_DIVISION_BY_ZERO",
        Trap::BadConversionToInteger => "BAD_CONVERSION_TO_INTEGER",
        Trap::UnreachableCodeReached => "UNREACHABLE_CODE_REACHED",
        Trap::Interrupt => "INTERRUPT",
        Trap::AlwaysTrapAdapter => "ALWAYS_TRAP_ADAPTER",
        Trap::OutOfFuel => "OUT_OF_FUEL",
        _ => "UNKNOWN",
    };

    env.get_static_field(
        "io/github/kawamuray/wasmtime/Trap",
        name,
        "Lio/github/kawamuray/wasmtime/Trap;",
    )?
    .l()
}
