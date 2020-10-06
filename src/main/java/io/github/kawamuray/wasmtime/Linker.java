package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access =  AccessLevel.PACKAGE)
public class Linker implements Disposable {
    @Getter
    private long innerPtr;

    public Linker(Store store) {
        this(newLinker(store.innerPtr()));
    }

    public void module(String moduleName, Module module) {
        nativeModule(moduleName, module.innerPtr());
    }

    public void define(String moduleName, String name, Extern extern) {
        nativeDefine(moduleName, name, extern);
    }

    public Extern getOneByName(String module, String name) {
        return nativeGetOneByName(module, name);
    }

    @Override
    public native void dispose();

    private static native long newLinker(long storePtr);

    private native void nativeModule(String moduleName, long modulePtr);

    private native void nativeDefine(String moduleName, String name, Extern externItem);

    private native Extern nativeGetOneByName(String module, String name);
}
