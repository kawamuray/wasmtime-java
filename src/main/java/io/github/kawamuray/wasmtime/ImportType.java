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

    @Getter(AccessLevel.PACKAGE)
    private Object typeObj;

    @Getter
    private String module;

    @Getter
    private String name;

    private void ensureType(ImportType.Type expected) {
        if (typeObj != expected) {
            throw new RuntimeException(
                String.format("ImportType expected to have type %s but is actually %s", expected, type));
        }
    }

    public Func func() {
        ensureType(ImportType.Type.FUNC);
        return (Func) typeObj;
    }

    public Global global() {
        ensureType(ImportType.Type.GLOBAL);
        return (Global) typeObj;
    }

    public Memory memory() {
        ensureType(ImportType.Type.MEMORY);
        return (Memory) typeObj;
    }

    public Table table() {
        ensureType(ImportType.Type.TABLE);
        return (Table) typeObj;
    }
}
