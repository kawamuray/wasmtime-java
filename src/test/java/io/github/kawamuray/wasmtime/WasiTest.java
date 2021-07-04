package io.github.kawamuray.wasmtime;

import io.github.kawamuray.wasmtime.wasi.Wasi;
import io.github.kawamuray.wasmtime.wasi.WasiConfig;
import org.junit.Test;

import java.io.File;

public class WasiTest {
    @Test
    public void testPreopenDirWithRelativePath(){
        WasiConfig wasiConfig = new WasiConfig(
                new String[]{},
                new WasiConfig.PreopenDir[]{
                        new WasiConfig.PreopenDir("./", "/")
                }
        );
        Store store = new Store();
        Wasi wasi = new Wasi(store, wasiConfig);
    }

    @Test
    public void testPreopenDirWithAbsolutePath(){
        WasiConfig wasiConfig = new WasiConfig(
                new String[]{},
                new WasiConfig.PreopenDir[]{
                        new WasiConfig.PreopenDir(new File("./").getAbsolutePath(), "/")
                }
        );
        Store store = new Store();
        Wasi wasi = new Wasi(store, wasiConfig);
    }
}
