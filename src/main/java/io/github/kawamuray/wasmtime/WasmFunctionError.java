package io.github.kawamuray.wasmtime;

import lombok.Getter;
import lombok.experimental.Accessors;

/**
 * The class to take similar role of anyhow::Error in wasmtime Rust library.
 */
@Accessors(fluent = true)
public class WasmFunctionError extends RuntimeException {
    public WasmFunctionError(String message) {
        super(message);
    }

    public static class I32ExitError extends WasmFunctionError {
        @Getter
        private final int exitCode;

        public I32ExitError(int exitCode) {
            super("exit code: " + exitCode);
            this.exitCode = exitCode;
        }
    }

    public static class TrapError extends WasmFunctionError {
        @Getter
        private final Trap trap;

        TrapError(Trap trap) {
            super("wasm trap: " + trap);
            this.trap = trap;
        }
    }
}
