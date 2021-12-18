package io.github.kawamuray.wasmtime;

public class InterruptHandle {
    private final long innerPtr;

    public InterruptHandle(long storePtr) {
        innerPtr = storePtr;
    }

    public native void interrupt();
}
