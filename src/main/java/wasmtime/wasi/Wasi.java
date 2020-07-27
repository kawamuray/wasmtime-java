package wasmtime.wasi;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;
import wasmtime.Disposable;
import wasmtime.Linker;
import wasmtime.Store;

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
