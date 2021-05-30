package io.github.kawamuray.wasmtime.wasi;

import lombok.Builder;
import lombok.Value;
import lombok.experimental.Accessors;
import java.util.Optional;

@Accessors(fluent = true)
@Builder
public class WasiConfig {
    private final String[] args;
    private final PreopenDir[] preopenDirs;
    private final Optional<String> stdoutFile;
    private final Optional<String> stderrFile;

    @Value
    public static class PreopenDir {
        String hostPath;
        String guestPath;
    }

    public WasiConfig(String[] args, PreopenDir[] preopenDirs) {
        this.args = args;
        this.preopenDirs = preopenDirs;
        this.stdoutFile = Optional.empty();
        this.stderrFile = Optional.empty();
    }

    public WasiConfig(String[] args, PreopenDir[] preopenDirs,
                      Optional<String> stdoutFile, Optional<String> stderrFile) {
        this.args = args;
        this.preopenDirs = preopenDirs;
        this.stdoutFile = stdoutFile;
        this.stderrFile = stderrFile;
    }
}
