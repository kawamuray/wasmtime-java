package io.github.kawamuray.wasmtime;

import lombok.AllArgsConstructor;

@AllArgsConstructor
public class InterruptHandle implements Disposable {
    private final long innerPtr;

    public native void interrupt();

    @Override
    public native void dispose();
}
