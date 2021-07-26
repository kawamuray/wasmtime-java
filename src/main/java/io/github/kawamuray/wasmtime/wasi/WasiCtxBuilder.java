package io.github.kawamuray.wasmtime.wasi;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.Map.Entry;

import io.github.kawamuray.wasmtime.NativeLibraryLoader;
import lombok.Value;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
public class WasiCtxBuilder {
    static {
        NativeLibraryLoader.init();
    }

    private final List<String[]> envs;
    private final List<String> args;
    private Path stdinPath;
    private boolean inheritStdin;
    private Path stdoutPath;
    private boolean inheritStdout;
    private Path stderrPath;
    private boolean inheritStderr;
    private final List<PreopenDir> preopenDirs;

    public WasiCtxBuilder() {
        envs = new ArrayList<>();
        args = new ArrayList<>();
        preopenDirs = new ArrayList<>();
    }

    @Value
    public static class PreopenDir {
        String hostPath;
        String guestPath;
    }

    public WasiCtxBuilder env(String var, String value) {
        envs.add(new String[]{var, value});
        return this;
    }

    public WasiCtxBuilder inheritEnv() {
        for (Entry<String, String> entry : System.getenv().entrySet()) {
            env(entry.getKey(), entry.getValue());
        }
        return this;
    }

    public WasiCtxBuilder arg(String arg) {
        args.add(arg);
        return this;
    }

    public WasiCtxBuilder args(Collection<String> args) {
        this.args.addAll(args);
        return this;
    }

    public WasiCtxBuilder stdin(Path path) {
        stdinPath = path;
        inheritStdin = false;
        return this;
    }

    public WasiCtxBuilder stdout(Path path) {
        stdoutPath = path;
        inheritStdout = false;
        return this;
    }

    public WasiCtxBuilder stderr(Path path) {
        stderrPath = path;
        inheritStderr = false;
        return this;
    }

    public WasiCtxBuilder inheritStdin() {
        stdinPath = null;
        inheritStdin = true;
        return this;
    }

    public WasiCtxBuilder inheritStdout() {
        stdoutPath = null;
        inheritStdout = true;
        return this;
    }

    public WasiCtxBuilder inheritStderr() {
        stderrPath = null;
        inheritStderr = true;
        return this;
    }

    public WasiCtxBuilder inheritStdio() {
        return inheritStdin().inheritStdout().inheritStderr();
    }

    public WasiCtxBuilder preopenedDir(Path dir, String guestPath) {
        preopenDirs.add(new PreopenDir(dir.toString(), guestPath));
        return this;
    }

    public WasiCtx build() {
        long wasiCtxPtr = nativeBuild(
                envs.toArray(), args.toArray(),
                inheritStdin, stdinPath == null ? null : stdinPath.toString(),
                inheritStdout, stdoutPath == null ? null : stdoutPath.toString(),
                inheritStderr, stderrPath == null ? null : stderrPath.toString(),
                preopenDirs.toArray());
        return new WasiCtx(wasiCtxPtr);
    }

    private static native long nativeBuild(
            Object[] envs, Object[] args,
            boolean inheritStdin, String stdinPath,
            boolean inheritStdout, String stdoutPath,
            boolean inheritStderr, String stderrPath,
            Object[] preopenDirs);
}
