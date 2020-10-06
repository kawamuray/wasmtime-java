package io.github.kawamuray.wasmtime;

import lombok.AllArgsConstructor;
import lombok.Value;
import lombok.experimental.Accessors;

@Value
@Accessors(fluent = true)
public class MemoryType {
    @Value
    @AllArgsConstructor
    public static class Limit {
        int min;
        int max; // optional

        public Limit(int min) {
            this(min, -1);
        }
    }

    Limit limit;
}
