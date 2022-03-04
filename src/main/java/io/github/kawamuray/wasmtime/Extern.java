package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Extern {
    public enum Type {
        UNKNOWN,
        FUNC,
        GLOBAL,
        TABLE,
        MEMORY,
    }

    public static final Extern UNKNOWN = new Extern(Type.UNKNOWN, null);

    @Getter
    private final Type type;
    private final Object value;

    public static Extern fromFunc(Func func) {
        return new Extern(Type.FUNC, func);
    }

    public static Extern fromMemory(Memory memory) {
        return new Extern(Type.MEMORY, memory);
    }

    public static Extern fromTable(Table table) {
        return new Extern(Type.TABLE, table);
    }

    public static Extern fromGlobal(Global table) {
        return new Extern(Type.GLOBAL, table);
    }

    private void ensureType(Type expected) {
        if (type != expected) {
            throw new RuntimeException(
                    String.format("extern expected to have type %s but is actually %s", expected, type));
        }
    }

    public Func func() {
        ensureType(Type.FUNC);
        return (Func) value;
    }

    public Global global() {
        ensureType(Type.GLOBAL);
        return (Global) value;
    }

    public Memory memory() {
        ensureType(Type.MEMORY);
        return (Memory) value;
    }

    public Table table() {
        ensureType(Type.TABLE);
        return (Table) value;
    }

}
