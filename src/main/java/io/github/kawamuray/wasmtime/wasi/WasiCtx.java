package io.github.kawamuray.wasmtime.wasi;

import java.nio.file.Path;
import java.util.Set;

import io.github.kawamuray.wasmtime.Disposable;
import io.github.kawamuray.wasmtime.Linker;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@AllArgsConstructor
public class WasiCtx implements Disposable {
    @Getter
    private long innerPtr;

    public static void addToLinker(Linker linker) {
        nativeAddToLinker(linker.innerPtr());
    }

    @Override
    public native void dispose();

    private static native void nativeAddToLinker(long linkerPtr);

    private static int fileCapBits(Set<FileCaps> caps) {
        return caps.stream().mapToInt(FileCaps::value).reduce(0, (left, right) -> left | right);
    }

    private static int dirCapBits(Set<DirCaps> caps) {
        return caps.stream().mapToInt(DirCaps::value).reduce(0, (left, right) -> left | right);
    }

    public void insertFile(int fd, Path file, Set<FileCaps> caps) {
        nativeInsertFile(fd, file.toString(), fileCapBits(caps));
    }

    private native void nativeInsertFile(int fd, String path, int fileCapBits);

    public void insertDir(int fd, Path dir, Set<DirCaps> dirCaps, Set<FileCaps> fileCaps, Path preopenPath) {
        nativeInsertDir(fd, dir.toString(), dirCapBits(dirCaps), fileCapBits(fileCaps), preopenPath.toString());
    }

    private native void nativeInsertDir(int fd, String dirPath, int dirCapBits, int fileCapBits, String preopenPath);

    public native void pushArg(String arg);

    public native void pushEnv(String var, String value);

    public void setStdin(Path path) {
        nativeSetStdin(path.toString());
    }

    private native void nativeSetStdin(String path);

    public void setStdout(Path path) {
        nativeSetStdout(path.toString());
    }

    private native void nativeSetStdout(String path);

    public void setStderr(Path path) {
        nativeSetStderr(path.toString());
    }

    private native void nativeSetStderr(String path);

    public void pushPreopenDir(Path dir, String guestPath) {
        nativePushPreopenDir(dir.toString(), guestPath);
    }

    private native void nativePushPreopenDir(String dirPath, String path);

    public long takeInnerPtr() {
        long ptr = innerPtr;
        innerPtr = 0;
        return ptr;
    }
}
