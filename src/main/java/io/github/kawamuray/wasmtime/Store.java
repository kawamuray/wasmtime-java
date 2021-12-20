package io.github.kawamuray.wasmtime;

import io.github.kawamuray.wasmtime.wasi.WasiCtx;
import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Store<T> implements Disposable {
    static {
        NativeLibraryLoader.init();
    }

    // TODO: can't make this package-private as long as we separate some packages like wasi
    @Getter
    private long innerPtr;

    public static Store<Void> withoutData() {
        return withoutData(null);
    }

    public static Store<Void> withoutData(WasiCtx wasiCtx) {
        return new Store<>(null, wasiCtx);
    }

    public Store(T data) {
        this(data, (WasiCtx) null);
    }

    public Store(T data, Engine engine) {
        this(data, engine, null);
    }

    public Store(T data, WasiCtx wasiCtx) {
        this(data, new Engine(), wasiCtx);
    }

    public Store(T data, Engine engine, WasiCtx wasiCtx) {
        this(newStore(engine.innerPtr(), data, wasiCtx == null ? 0 : wasiCtx.takeInnerPtr()));
    }

    public Engine engine() {
        return new Engine(enginePtr());
    }

    @SuppressWarnings("unchecked")
    public T data() {
        return (T) storedData();
    }

    public InterruptHandle interruptHandle() {
        return new InterruptHandle(interruptHandlePtr());
    }

    @Override
    public native void dispose();

    private static native long newStore(long enginePtr, Object data, long wasiCtxPtr);

    private native long enginePtr();

    private native Object storedData();

    public native void gc();

    private native long interruptHandlePtr();
}
