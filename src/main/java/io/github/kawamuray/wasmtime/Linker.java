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

    public <T> Collection<ExternItem> externs(Store<T> store, String module) {
        Set<ExternItem> set = new HashSet<>();
        for (String name : nativeExterns(store.innerPtr(), module)) {
            try {
                set.add(new ExternItem(name, nativeGet(store.innerPtr(), module, name)));
            } catch(Exception e) {
                // ignore native errors for externs of unimplemented type
                log.debug("Encountered unsupported extern: " + name + " in module: " + module);
            }
        }
        return set;
    }

    public <T> Set<String> modules(Store<T> store) {
        return new HashSet<>(Arrays.asList(nativeModules(store.innerPtr())));
    }

    @Override
    public native void dispose();

    private static native long newLinker(long enginePtr);

    private native void nativeModule(long storePtr, String moduleName, long modulePtr);

    private native void nativeDefine(String moduleName, String name, Extern externItem);

    private native Extern nativeGet(long storePtr, String module, String name);

    private native String[] nativeExterns(long storePtr, String module);

    private native String[] nativeModules(long storePtr);
}
