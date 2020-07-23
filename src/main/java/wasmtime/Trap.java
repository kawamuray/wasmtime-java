package wasmtime;

import lombok.AllArgsConstructor;

@AllArgsConstructor
public class Trap {
    public enum Type {
        MESSAGE,
        I32_EXIT,
    }

    private final Type type;
    private final String message;
    private final int exitCode;

    public static Trap fromMessage(String message) {
        return new Trap(Type.MESSAGE, message, 0);
    }

    public static Trap fromExitCode(int exitCode) {
        return new Trap(Type.I32_EXIT, null, exitCode);
    }

    public static Trap fromException(Throwable e) {
        return fromMessage(e.getMessage());
    }
}
