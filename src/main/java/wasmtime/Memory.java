package wasmtime;

import java.nio.ByteBuffer;

import lombok.AllArgsConstructor;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor
public class Memory implements Disposable {
    private long innerPtr;

    public Memory(Store store, MemoryType type) {
        this(newMemory(store.innerPtr(), type.limit().min(), type.limit().max()));
    }

    public native ByteBuffer buffer();

    public native long dataSize();

    public native int size();

    @Override
    public native void dispose();

    private static native long newMemory(long storePtr, int min, int max);
}
