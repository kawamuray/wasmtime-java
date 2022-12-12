package io.github.kawamuray.wasmtime;

public enum Trap {
    UNKNOWN,
    // The current stack space was exhausted.
    STACK_OVERFLOW,
    // An out-of-bounds memory access.
    MEMORY_OUT_OF_BOUNDS,
    // A wasm atomic operation was presented with a not-naturally-aligned linear-memory address.
    HEAP_MISALIGNED,
    // An out-of-bounds access to a table.
    TABLE_OUT_OF_BOUNDS,
    // Indirect call to a null table entry.
    INDIRECT_CALL_TO_NULL,
    // Signature mismatch on indirect call.
    BAD_SIGNATURE,
    // An integer arithmetic operation caused an overflow.
    INTEGER_OVERFLOW,
    // An integer division by zero.
    INTEGER_DIVISION_BY_ZERO,
    // Failed float-to-int conversion.
    BAD_CONVERSION_TO_INTEGER,
    // Code that was supposed to have been unreachable was reached.
    UNREACHABLE_CODE_REACHED,
    // Execution has potentially run too long and may be interrupted.
    INTERRUPT,
    // When the `component-model` feature is enabled this trap represents a
    // function that was `canon lift`'d, then `canon lower`'d, then called.
    // This combination of creation of a function in the component model
    // generates a function that always traps and, when called, produces this
    // flavor of trap.
    ALWAYS_TRAP_ADAPTER,
    /// When wasm code is configured to consume fuel and it runs out of fuel
    /// then this trap will be raised.
    ///
    /// For more information see
    /// [`Config::consume_fuel`](crate::Config::consume_fuel).
    OUF_OF_FUEL,
}
