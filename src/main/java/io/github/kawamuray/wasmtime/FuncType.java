package io.github.kawamuray.wasmtime;

import lombok.AllArgsConstructor;
import lombok.Value;

@Value
@AllArgsConstructor
public class FuncType {
    private static final Val.Type[] EMPTY_TYPES = new Val.Type[0];

    Val.Type[] params;
    Val.Type[] results;

    public static FuncType empty() {
        return new FuncType(EMPTY_TYPES, EMPTY_TYPES);
    }

    public static FuncType emptyParams(Val.Type... resultTypes) {
        return new FuncType(EMPTY_TYPES, resultTypes);
    }

    public static FuncType emptyResults(Val.Type... paramTypes) {
        return new FuncType(paramTypes, EMPTY_TYPES);
    }
}
