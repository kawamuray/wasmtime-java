package wasmtime;

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

    public Optional<Func> getFunc(String name) {
        long ptr = nativeGetFunc(name);
        return ptr == 0 ? Optional.empty() : Optional.of(new Func(ptr));
    }

    public Optional<Memory> getMemory(String name) {
        long ptr = nativeGetMemory(name);
        return ptr == 0 ? Optional.empty() : Optional.of(new Memory(ptr));
    }

    @Override
    public native void dispose();

    private native long nativeGetFunc(String name);

    private native long nativeGetMemory(String name);
}
