package io.github.kawamuray.wasmtime;

public class WasmtimeException extends RuntimeException {
    public WasmtimeException(String message) {
        super(message);
    }
}
