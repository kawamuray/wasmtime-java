package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class ImportType {
    public enum Type {
        UNKNOWN,
        FUNC,
        GLOBAL,
        TABLE,
        MEMORY,
    }

    @Getter
    private Type type;

    @Getter
    private Object typeObj;

    @Getter
    private String module;

    @Getter
    private String name;
}
