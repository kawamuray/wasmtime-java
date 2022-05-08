package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class ExportType {
    public enum Type {
        FUNC,
        GLOBAL,
        TABLE,
        MEMORY,
        // TODO: Currently Unsupported
        INSTANCE,
        MODULE;
    }
    @Getter
    private final Type type;

    @Getter(AccessLevel.PACKAGE)
    private final Object typeObj;

    @Getter
    private final String name;

    private void ensureType(Type expected) {
        if (type != expected) {
            throw new RuntimeException(
                String.format("ImportType expected to have type %s but is actually %s", expected, type));
        }
    }

    public FuncType func() {
        ensureType(ExportType.Type.FUNC);
        return (FuncType) typeObj;
    }

    public GlobalType global() {
        ensureType(ExportType.Type.GLOBAL);
        return (GlobalType) typeObj;
    }

    public MemoryType memory() {
        ensureType(ExportType.Type.MEMORY);
        return (MemoryType) typeObj;
    }

    public TableType table() {
        ensureType(ExportType.Type.TABLE);
        return (TableType) typeObj;
    }
}
