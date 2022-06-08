package io.github.kawamuray.wasmtime;

import lombok.AllArgsConstructor;
import lombok.Value;
import lombok.experimental.Accessors;

@Value
@Accessors(fluent = true)
@AllArgsConstructor
public class TableType {
    Val.Type element;
    int minimum;
    int maximum;

    public TableType(Val.Type element, int minimum) {
        this(element, minimum, -1);
    }
}
