package io.github.kawamuray.wasmtime.wasi;

import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
public enum DirCaps {
    CREATE_DIRECTORY(0b1),
    CREATE_FILE(0b10),
    LINK_SOURCE(0b100),
    LINK_TARGET(0b1000),
    OPEN(0b10000),
    READDIR(0b100000),
    READLINK(0b1000000),
    RENAME_SOURCE(0b10000000),
    RENAME_TARGET(0b100000000),
    SYMLINK(0b1000000000),
    REMOVE_DIRECTORY(0b10000000000),
    UNLINK_FILE(0b100000000000),
    PATH_FILESTAT_GET(0b1000000000000),
    PATH_FILESTAT_SET_TIMES(0b10000000000000),
    FILESTAT_GET(0b100000000000000),
    FILESTAT_SET_TIMES(0b1000000000000000),
    ;

    @Getter
    private final int value;

    DirCaps(int value) {
        this.value = value;
    }
}
