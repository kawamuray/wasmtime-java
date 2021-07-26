package io.github.kawamuray.wasmtime;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class StoreTest {
    @Test
    public void testCreate() {
        Store<Void> store = Store.withoutData();
        store.close();
    }

    @Test
    public void testEngine() {
        try (Store<Void> store = Store.withoutData()) {
            Engine engine = store.engine();
            engine.close();
        }
    }

    @Test(expected = NullPointerException.class)
    public void testUseAfterFree() {
        Store<Void> store = Store.withoutData();
        store.close();
        store.engine(); // UAF
    }


    private class StoreData {
        int i;

        StoreData(int i) {
            this.i = i;
        }
    }

    @Test
    public void testStoreData() {
        Store<StoreData> store = new Store<>(new StoreData(1234));
        assertEquals(1234, store.data().i);
        store.close();
    }
}
