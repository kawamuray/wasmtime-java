package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Module implements Disposable {
    @Getter(AccessLevel.PACKAGE)
    private final long innerPtr;

    public Module(Engine engine, byte[] bytes) {
        this(newModule(engine.innerPtr(), bytes));
    }

    public static Module fromFile(Engine engine, String fileName) {
        return new Module(newFromFile(engine.innerPtr(), fileName));
    }

    public static Module fromBinary(Engine engine, byte[] bytes) {
        return new Module(newFromBinary(engine.innerPtr(), bytes));
    }

    public native ImportType[] imports();

    public native ExportType[] exports();

    @Override
    public native void dispose();

    private static native long newModule(long enginePtr, byte[] bytes);

    private static native long newFromFile(long enginePtr, String fileName);

    private static native long newFromBinary(long enginePtr, byte[] bytes);
}
