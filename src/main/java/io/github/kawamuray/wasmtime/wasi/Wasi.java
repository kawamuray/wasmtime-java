package io.github.kawamuray.wasmtime.wasi;

import io.github.kawamuray.wasmtime.Store;
import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;
import io.github.kawamuray.wasmtime.Disposable;
import io.github.kawamuray.wasmtime.Linker;

@Accessors(fluent = true)
@AllArgsConstructor
public class Wasi implements Disposable {
    @Getter(AccessLevel.PACKAGE)
    private long innerPtr;

    public Wasi(Store store, WasiConfig ctx) {
        this(newWasi(store.innerPtr(), ctx));
    }

    public void addToLinker(Linker linker) {
        nativeAddToLinker(linker.innerPtr());
    }

    @Override
    public native void dispose();

    private static native long newWasi(long storePtr, WasiConfig config);

    private native void nativeAddToLinker(long linkerPtr);
}
