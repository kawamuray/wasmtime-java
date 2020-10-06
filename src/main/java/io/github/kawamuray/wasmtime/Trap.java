package io.github.kawamuray.wasmtime;

import lombok.AllArgsConstructor;
import lombok.NonNull;

@AllArgsConstructor
public class Trap {
    public enum Type {
        MESSAGE,
        I32_EXIT,
    }

    private final Type type;
    private final String message;
    private final int exitCode;

    public static Trap fromMessage(@NonNull String message) {
        return new Trap(Type.MESSAGE, message, 0);
    }

    public static Trap fromExitCode(int exitCode) {
        return new Trap(Type.I32_EXIT, null, exitCode);
    }

    public static Trap fromException(Throwable e) {
        return fromMessage(String.valueOf(e));
    }
}
