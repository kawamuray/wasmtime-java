package io.github.kawamuray.wasmtime;

import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@AllArgsConstructor
@Accessors(fluent = true)
public class TrapException extends RuntimeException {
    @Getter
    private final Trap trap;
}
