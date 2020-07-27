package wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Extern {
    public enum Type {
        FUNC,
        GLOBAL,
        TABLE,
        MEMORY,
    }

    @Getter
    private final Type type;
    private final Object value;

    public static Extern fromFunc(Func func) {
        return new Extern(Type.FUNC, func);
    }

    public static Extern fromMemory(Memory memory) {
        return new Extern(Type.MEMORY, memory);
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

    public Memory memory() {
        ensureType(Type.MEMORY);
        return (Memory) value;
    }
}
