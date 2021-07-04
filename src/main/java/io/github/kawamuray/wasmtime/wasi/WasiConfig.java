package io.github.kawamuray.wasmtime.wasi;

import lombok.Builder;
import lombok.Value;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@Builder
public class WasiConfig {
    private final String[] args;
    private final PreopenDir[] preopenDirs;
    private final String stdoutFile;
    private final String stderrFile;

    @Value
    public static class PreopenDir {
        String hostPath;
        String guestPath;
    }

    public WasiConfig(String[] args, PreopenDir[] preopenDirs) {
        this(args, preopenDirs, null, null);
    }

    public WasiConfig(String[] args, PreopenDir[] preopenDirs,
                      String stdoutFile, String stderrFile) {
        this.args = args;
        this.preopenDirs = preopenDirs;
        this.stdoutFile = stdoutFile;
        this.stderrFile = stderrFile;
    }
}
