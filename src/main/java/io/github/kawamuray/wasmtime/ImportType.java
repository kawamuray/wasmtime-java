package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class ImportType implements Disposable {
    @Getter(AccessLevel.PACKAGE)
    private final long innerPtr;

    public native String module();

    public native String name();

    public native String ty();

    @Override
    public native void dispose();
}
