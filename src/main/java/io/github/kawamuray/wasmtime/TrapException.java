package io.github.kawamuray.wasmtime;

import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
public class TrapException extends WasmtimeException {
    @Getter
    private final Trap trap;

    TrapException(Trap trap) {
        super(trap.message());
        this.trap = trap;
    }
}
