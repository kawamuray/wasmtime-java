package io.github.kawamuray.wasmtime;

public enum ExternType {
    FUNC,
    GLOBAL,
    TABLE,
    MEMORY,
    // TODO: Currently Unsupported
    INSTANCE,
    MODULE
}
