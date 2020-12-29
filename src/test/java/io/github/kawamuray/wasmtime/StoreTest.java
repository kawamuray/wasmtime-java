package io.github.kawamuray.wasmtime;

import org.junit.Test;

public class StoreTest {
    @Test
    public void testCreate() {
        Store store = new Store();
        store.close();
    }

    @Test
    public void testEngine() {
        try (Store store = new Store()) {
            Engine engine = store.engine();
            engine.close();
        }
    }

    @Test(expected = NullPointerException.class)
    public void testUseAfterFree() {
        Store store = new Store();
        store.close();
        store.engine(); // UAF
    }
}
