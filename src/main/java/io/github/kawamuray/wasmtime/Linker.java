package io.github.kawamuray.wasmtime;

import java.util.Optional;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access =  AccessLevel.PACKAGE)
public class Linker implements Disposable {
    @Getter
    private long innerPtr;

    public Linker(Engine engine) {
        this(newLinker(engine.innerPtr()));
    }

    public <T> void module(Store<T> store, String moduleName, Module module) {
        nativeModule(store.innerPtr(), moduleName, module.innerPtr());
    }

    public void define(String moduleName, String name, Extern extern) {
        nativeDefine(moduleName, name, extern);
    }

    public <T> Optional<Extern> get(Store<T> store, String module, String name) {
        return Optional.ofNullable(nativeGet(store.innerPtr(), module, name));
    }

    @Override
    public native void dispose();

    private static native long newLinker(long enginePtr);

    private native void nativeModule(long storePtr, String moduleName, long modulePtr);

    private native void nativeDefine(String moduleName, String name, Extern externItem);

    private native Extern nativeGet(long storePtr, String module, String name);
}
