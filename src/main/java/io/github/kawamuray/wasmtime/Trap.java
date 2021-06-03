package io.github.kawamuray.wasmtime;

import lombok.AllArgsConstructor;
import lombok.NonNull;
import lombok.Value;
import lombok.experimental.Accessors;

@Value
@Accessors(fluent = true)
@AllArgsConstructor
public class Trap {
    public enum Type {
        MESSAGE,
        I32_EXIT,
    }

    Type type;
    String message;
    int exitCode;

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
