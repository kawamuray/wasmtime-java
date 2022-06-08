package io.github.kawamuray.wasmtime;

import lombok.AllArgsConstructor;
import lombok.Value;
import lombok.experimental.Accessors;

@Value
@Accessors(fluent = true)
@AllArgsConstructor
public class MemoryType {
    long minimum;
    long maximum;
    boolean is64;

    public MemoryType(long minimum, boolean is64) {
        this(minimum, -1, is64);
    }
}
