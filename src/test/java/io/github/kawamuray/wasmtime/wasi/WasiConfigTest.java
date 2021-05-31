package io.github.kawamuray.wasmtime.wasi;
import io.github.kawamuray.wasmtime.Store;
import org.junit.Test;
import java.io.File;
import java.io.IOException;

public class WasiConfigTest {
    @Test
    public void testNewConfig() {
        WasiConfig config = new WasiConfig(new String[0], new WasiConfig.PreopenDir[0]);
        try (Store store = new Store();
            Wasi wasi = new Wasi(store, config)
        ) {}
    }

    @Test
    public void testNewConfigWithArgs() {
        WasiConfig config = new WasiConfig(new String[]{"foo", "bar"}, new WasiConfig.PreopenDir[0]);
        try (Store store = new Store();
            Wasi wasi = new Wasi(store, config)
        ) {}
    }

    @Test
    public void testNewConfigWithStdOutput() throws IOException {
        File f = File.createTempFile("wasmtime-jni-wasi-config", "");
        WasiConfig config = new WasiConfig(
            new String[0], new WasiConfig.PreopenDir[0],
            f.getPath(), f.getPath()
        );
        try (Store store = new Store();
            Wasi wasi = new Wasi(store, config)
        ) {}
        f.delete();
    }
}
