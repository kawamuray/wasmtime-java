package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Global implements Disposable {

    @Getter(AccessLevel.PACKAGE)
    private final long innerPtr;

    public <T> Val get(Store<T> store) {
        return nativeGet(store.innerPtr());
    }

    public <T> void set(Store<T> store, Val val) {
        if (!isMutable(store)) {
            throw new IllegalStateException("Cannot set immutable global");
        }
        nativeSet(store.innerPtr(), val);
    }

    public <T> boolean isMutable(Store<T> store) {
        return nativeMutable(store.innerPtr());
    }

    @Override
    public native void dispose();

    private native Val nativeGet(long storePtr);

    private native void nativeSet(long storePtr, Val val);

    private native boolean nativeMutable(long storePtr);

}
