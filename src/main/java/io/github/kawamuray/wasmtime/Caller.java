package io.github.kawamuray.wasmtime;

import java.util.Optional;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Caller<T> implements Disposable {
    @Getter(AccessLevel.PACKAGE)
    private long innerPtr;

    public Optional<Extern> getExport(String name) {
        return Optional.ofNullable(nativeGetExport(name));
    }

    public native T data();

    @Override
    public void dispose() {
        // We don't need to drop the instance referred by this pointer because while calling trampoline function
        // Rust still holds ownership of the Caller and drops it once it returns.
        // Just clear innerPtr here to prevent use-after-free.
        innerPtr = 0;
    }

    private native Extern nativeGetExport(String name);
}
