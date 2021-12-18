package io.github.kawamuray.wasmtime;

public class InterruptHandle {
    private final long innerPtr;

    public InterruptHandle(long interruptHandlerPtr) {
        innerPtr = interruptHandlerPtr;
    }

    public native void interrupt();
}
