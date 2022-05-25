package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.experimental.Accessors;

/**
 * An Engine which is a global context for compilation and management of wasm modules.
 * <p>
 * Engines store global configuration preferences such as compilation settings, enabled features, etc.
 * You'll likely only need at most one of these for a program.
 *
 * @see <a href="https://docs.wasmtime.dev/api/wasmtime/struct.Engine.html">Rust Documentation</a>
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
