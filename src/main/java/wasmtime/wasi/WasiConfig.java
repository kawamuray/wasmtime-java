package wasmtime.wasi;

import lombok.experimental.Accessors;

@Accessors(fluent = true)
public class WasiConfig {
    private final String[] args;

    public WasiConfig(String[] args) {
        this.args = args;
    }
}
