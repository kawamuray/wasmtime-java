package io.github.kawamuray.wasmtime;

import lombok.Value;
import lombok.experimental.Accessors;

@Value
@Accessors(fluent = true)
public class TableType {
    Val.Type element;
    MemoryType.Limit limit;
}
