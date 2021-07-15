package io.github.kawamuray.wasmtime;

import java.util.Collection;
import java.util.Optional;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Instance implements Disposable {
    private static final Extern[] EMPTY_EXTERNS = new Extern[0];
    @Getter(AccessLevel.PACKAGE)
    private long innerPtr;

    public Instance(Store store, Module module, Collection<Extern> imports) {
        this(newInstance(store.innerPtr(), module.innerPtr(), imports.toArray(EMPTY_EXTERNS)));
    }

    private static native long newInstance(long storePtr, long modulePtr, Extern[] externs);

    public <T> Optional<Func> getFunc(Store<T> store, String name) {
        long ptr = nativeGetFunc(store.innerPtr(), name);
        return ptr == 0 ? Optional.empty() : Optional.of(new Func(ptr));
    }

    public <T> Optional<Memory> getMemory(Store<T> store, String name) {
        long ptr = nativeGetMemory(store.innerPtr(), name);
        return ptr == 0 ? Optional.empty() : Optional.of(new Memory(ptr));
    }

    @Override
    public native void dispose();

    private native long nativeGetFunc(long storePtr, String name);

    private native long nativeGetMemory(long storePtr, String name);
}
