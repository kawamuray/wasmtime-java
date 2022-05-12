package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class ImportType {
    @Getter
    private final ExternType type;

    @Getter(AccessLevel.PACKAGE)
    private final Object typeObj;

    @Getter
    private final String module;

    @Getter
    private final String name;

    private void ensureType(ExternType expected) {
        if (type != expected) {
            throw new RuntimeException(
                String.format("ImportType expected to have type %s but is actually %s", expected, type));
        }
    }

    public FuncType func() {
        ensureType(ExternType.FUNC);
        return (FuncType) typeObj;
    }

    public GlobalType global() {
        ensureType(ExternType.GLOBAL);
        return (GlobalType) typeObj;
    }

    public MemoryType memory() {
        ensureType(ExternType.MEMORY);
        return (MemoryType) typeObj;
    }

    public TableType table() {
        ensureType(ExternType.TABLE);
        return (TableType) typeObj;
    }
}
