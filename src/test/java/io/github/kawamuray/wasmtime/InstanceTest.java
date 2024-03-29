package io.github.kawamuray.wasmtime;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

import java.util.Collections;

import org.junit.Test;

public class InstanceTest {
    private static final byte[] WAT_BYTES_ADD = ("(module"
            + "  (memory (export \"memory\") 2 3)"
            + "  (func (export \"add\") (param $p1 i32) (param $p2 i32) (result i32)"
            + "    local.get $p1"
            + "    local.get $p2"
            + "    i32.add)"
            + ')').getBytes();

    @Test
    public void testCreateDispose() {
        try (Store<Void> store = Store.withoutData();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_ADD)) {
            Instance instance = new Instance(store, module, Collections.emptyList());
            instance.close();
        }
    }

    @Test
    public void testGetFunc() {
        try (Store<Void> store = Store.withoutData();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_ADD);
             Instance instance = new Instance(store, module, Collections.emptyList())) {
            assertTrue(instance.getFunc(store, "add").isPresent());
            assertFalse(instance.getFunc(store, "absent").isPresent());
        }
    }

    @Test
    public void testGetMemory() {
        try (Store<Void> store = Store.withoutData();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_ADD);
             Instance instance = new Instance(store, module, Collections.emptyList())) {
            assertTrue(instance.getMemory(store, "memory").isPresent());
            assertFalse(instance.getMemory(store, "add").isPresent());
        }
    }
}
