package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.experimental.Accessors;

/**
 * An Engine which is a global context for compilation and management of wasm modules.
 * <p>
 * An engine can be safely shared across threads and is a cheap cloneable handle to the actual engine.
 * The engine itself will be deallocated once all references to it have gone away.
 * <p>
 * Engines store global configuration preferences such as compilation settings, enabled features, etc.
 * You'll likely only need at most one of these for a program.
 */
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
