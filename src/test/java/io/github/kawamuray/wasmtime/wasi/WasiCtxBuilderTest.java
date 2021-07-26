package io.github.kawamuray.wasmtime.wasi;

import io.github.kawamuray.wasmtime.Store;

import org.junit.Test;

import java.io.File;
import java.io.IOException;
import java.util.Arrays;

public class WasiCtxBuilderTest {
    @Test
    public void testNewConfig() {
        WasiCtx ctx = new WasiCtxBuilder().build();
        Store<Void> store = Store.withoutData(ctx);
        store.close();
    }

    @Test
    public void testNewConfigWithArgs() {
        WasiCtx ctx = new WasiCtxBuilder().args(Arrays.asList("foo", "bar")).build();
        Store<Void> store = Store.withoutData(ctx);
        store.close();
    }

    @Test
    public void testNewConfigWithStdOutput() throws IOException {
        File f = File.createTempFile("wasmtime-jni-wasi-config", "");
        WasiCtx ctx = new WasiCtxBuilder().stdout(f.toPath()).stderr(f.toPath()).build();
        Store<Void> store = Store.withoutData(ctx);
        store.close();
        f.delete();
    }
}
