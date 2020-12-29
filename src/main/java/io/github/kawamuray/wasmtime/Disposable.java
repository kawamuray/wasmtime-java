package io.github.kawamuray.wasmtime;

public interface Disposable extends AutoCloseable {
    @Override
    default void close() {
        dispose();
    }

    void dispose();
}
