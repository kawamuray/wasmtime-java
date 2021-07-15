package io.github.kawamuray.wasmtime;

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

    public Store(T data) {
        this(newStore(data));
    }

    public Store(T data, Engine engine){
        this(newStoreWithEngine(data, engine));
    }

    public Engine engine() {
        return new Engine(enginePtr());
    }

    @SuppressWarnings("unchecked")
    public T data() {
        return (T) storedData();
    }

    @Override
    public native void dispose();

    private static native long newStore(Object data);

    private static native long newStoreWithEngine(Object data, Engine engine);

    private native long enginePtr();

    private native Object storedData();
}
