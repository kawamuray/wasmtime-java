package wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Store implements Disposable {
    static {
        NativeLibraryLoader.load();
    }

    // TODO: can't make this package-private as long as we separate some packages like wasi
    @Getter
    private long innerPtr;

    public Store() {
        this(newStore());
    }

    public Engine engine() {
        return new Engine(enginePtr());
    }

    @Override
    public native void dispose();

    private static native long newStore();

    private native long enginePtr();
}
