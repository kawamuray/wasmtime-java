package io.github.kawamuray.wasmtime;

public class InterruptHandle {
    private final long innerPtr;

    public InterruptHandle(long storePtr) {
        innerPtr = newInterruptHandle(storePtr);
    }

    public native void interrupt();

    private native long newInterruptHandle(long storePtr);
}
