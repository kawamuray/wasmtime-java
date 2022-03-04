package io.github.kawamuray.wasmtime;

import java.util.Arrays;
import java.util.Collection;
import java.util.HashSet;
import java.util.Optional;
import java.util.Set;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;
import lombok.extern.slf4j.Slf4j;

@Accessors(fluent = true)
@AllArgsConstructor(access =  AccessLevel.PACKAGE)
@Slf4j
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

    public <T> Collection<ExternItem> externs(Store<T> store) {
        ExternItem[] items = nativeExterns(store.innerPtr());
        return Arrays.asList(items);
    }

    public <T> Collection<ExternItem> externsOfModule(Store<T> store, String module) {
        Collection<ExternItem> items = new HashSet<>(externs(store));
        items.removeIf(item -> !item.module().equals(module));
        return items;
    }

    public <T> Set<String> modules(Store<T> store) {
        Set<String> modules = new HashSet<>();
        for (ExternItem item : externs(store)) {
            modules.add(item.module());
        }
        return modules;
    }

    @Override
    public native void dispose();

    private static native long newLinker(long enginePtr);

    private native void nativeModule(long storePtr, String moduleName, long modulePtr);

    private native void nativeDefine(String moduleName, String name, Extern externItem);

    private native Extern nativeGet(long storePtr, String module, String name);

    private native ExternItem[] nativeExterns(long storePtr);
}
