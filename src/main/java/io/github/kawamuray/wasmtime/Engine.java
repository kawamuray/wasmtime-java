package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@EqualsAndHashCode
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Engine implements Disposable {
    static {
        NativeLibraryLoader.init();
    }

    @Getter(AccessLevel.PACKAGE)
    private long innerPtr;

    public Engine() {
        this(newEngine());
    }

    public Engine(Config config){
        this(newEngineWithConfig(config));
    }

    @Override
    public native void dispose();

    private static native long newEngine();

    private static native long newEngineWithConfig(Config config);
}
