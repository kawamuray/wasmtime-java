package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class ImportType {
    public enum Type {
        FUNC,
        GLOBAL,
        TABLE,
        MEMORY,
        // TODO: Currently Unsupported
        INSTANCE,
        MODULE
    }

    @Getter
    private final Type type;

    @Getter(AccessLevel.PACKAGE)
    private final Object typeObj;

    @Getter
    private final String module;

    @Getter
    private final String name;

    private void ensureType(ImportType.Type expected) {
        if (type != expected) {
            throw new RuntimeException(
                String.format("ImportType expected to have type %s but is actually %s", expected, type));
        }
    }

    public FuncType func() {
        ensureType(ImportType.Type.FUNC);
        return (FuncType) typeObj;
    }

    public GlobalType global() {
        ensureType(ImportType.Type.GLOBAL);
        return (GlobalType) typeObj;
    }

    public MemoryType memory() {
        ensureType(ImportType.Type.MEMORY);
        return (MemoryType) typeObj;
    }

    public Table table() {
        ensureType(ImportType.Type.TABLE);
        return (Table) typeObj;
    }
}
