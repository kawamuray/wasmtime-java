package io.github.kawamuray.wasmtime;

import io.github.kawamuray.wasmtime.wasi.WasiCtx;
import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

/**
 * A Store is a collection of WebAssembly instances and host-defined state.
 * <p>
 * All WebAssembly instances and items will be attached to and refer to a Store. For example instances, functions,
 * globals, and tables are all attached to a Store. Instances are created by instantiating a Module within a Store.
 * <p>
 * A Store is intended to be a short-lived object in a program. No form of GC is implemented at this time so once an
 * instance is created within a Store it will not be deallocated until the Store itself is dropped. This makes Store
 * unsuitable for creating an unbounded number of instances in it because Store will never release this memory. It’s
 * recommended to have a Store correspond roughly to the lifetime of a "main instance" that an embedding is interested
 * in executing.
 *
 * <h1>Type parameter T</h1>
 * <p>
 * Each Store has a type parameter T associated with it. This T represents state defined by the host.
 * This state will be accessible through the Caller type that host-defined functions get access to.
 * This T is suitable for storing Store-specific information which imported functions may want access to.
 * <p>
 * The data T can be accessed through methods like Store::data and Store::data_mut.
 * Stores, contexts, oh my
 * <p>
 * Most methods in Wasmtime take something of the form AsContext or AsContextMut as the first argument.
 * These two traits allow ergonomically passing in the context you currently have to any method.
 * The primary two sources of contexts are:
 * <p>
 * Store&lt;T&gt;
 * Caller&lt;'_, T&gt;
 * <p>
 * corresponding to what you create and what you have access to in a host function. You can also explicitly acquire
 * a StoreContext or StoreContextMut and pass that around as well.
 * <p>
 * Note that all methods on Store are mirrored onto StoreContext, StoreContextMut, and Caller. This way no matter what
 * form of context you have you can call various methods, create objects, etc.
 *
 * <h1>Stores and Default</h1>
 * <p>
 * You can create a store with default configuration settings using Store::default(). This will create a
 * brand new Engine with default configuration (see Config for more information).
 *
 * @param <T> State defined by the host
 */
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
