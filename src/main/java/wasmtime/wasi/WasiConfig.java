package wasmtime.wasi;

import lombok.Builder;
import lombok.Value;
import lombok.experimental.Accessors;

@Accessors(fluent = true)
@Builder
public class WasiConfig {
    private final String[] args;
    private final PreopenDir[] preopenDirs;

    @Value
    public static class PreopenDir {
        String hostPath;
        String guestPath;
    }

    public WasiConfig(String[] args, PreopenDir[] preopenDirs) {
        this.args = args;
        this.preopenDirs = preopenDirs;
    }
}
