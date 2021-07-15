package io.github.kawamuray.wasmtime.wasi;

import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
public enum FileCaps {
    DATASYNC(0b1),
    READ(0b10),
    SEEK(0b100),
    FDSTAT_SET_FLAGS(0b1000),
    SYNC(0b10000),
    TELL(0b100000),
    WRITE(0b1000000),
    ADVISE(0b10000000),
    ALLOCATE(0b100000000),
    FILESTAT_GET(0b1000000000),
    FILESTAT_SET_SIZE(0b10000000000),
    FILESTAT_SET_TIMES(0b100000000000),
    POLL_READWRITE(0b1000000000000),
    ;

    @Getter
    private final int value;

    FileCaps(int value) {
        this.value = value;
    }
}
