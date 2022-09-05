package io.github.kawamuray.wasmtime;

import lombok.AllArgsConstructor;
import lombok.NonNull;
import lombok.Value;
import lombok.experimental.Accessors;

@Value
@Accessors(fluent = true)
@AllArgsConstructor
public class Trap {
    public enum Reason {
        MESSAGE,
        I32_EXIT,
        INSTRUCTION_TRAP,
    }

    public enum TrapCode {
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
    }

    Reason reason;
    String message;
    int exitCode;
    TrapCode trapCode;

    public static Trap fromMessage(@NonNull String message) {
        return new Trap(Reason.MESSAGE, message, 0, null);
    }

    public static Trap fromExitCode(int exitCode) {
        return new Trap(Reason.I32_EXIT, null, exitCode, null);
    }

    public static Trap fromTrapCode(TrapCode trapCode) {
        return new Trap(Reason.INSTRUCTION_TRAP, null, 0, trapCode);
    }

    public static Trap fromException(Throwable e) {
        return fromMessage(String.valueOf(e));
    }
}
