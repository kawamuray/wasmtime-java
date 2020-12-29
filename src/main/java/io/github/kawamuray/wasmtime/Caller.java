package io.github.kawamuray.wasmtime;

import java.util.Optional;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Caller implements Disposable {
    @Getter(AccessLevel.PACKAGE)
    private long innerPtr;

    public Optional<Extern> getExport(String name) {
        return Optional.ofNullable(nativeGetExport(name));
    }

    public Store store() {
        return new Store(storePtr());
    }

    @Override
    public void dispose() {
        // We don't need to drop the instance referred by this pointer because while calling trampoline function
        // Rust still holds ownership of the Caller and drops it once it returns.
        // Just clear innerPtr here to prevent use-after-free.
        innerPtr = 0;
    }

    private static native Extern nativeGetExport(String name);

    private native long storePtr();
}
