package io.github.kawamuray.wasmtime;

import static io.github.kawamuray.wasmtime.WasmValType.I32;
import static org.junit.Assert.assertEquals;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import java.nio.ByteBuffer;
import java.util.Collections;

import io.github.kawamuray.wasmtime.WasmFunctions.Consumer2;
import io.github.kawamuray.wasmtime.WasmFunctions.Function1;

public class MemoryTest {
    private static final long PAGE_SIZE = 64 * 1024; // 64kb

    private static final byte[] WAT_BYTES = ("(module"
            + "  (memory (export \"memory\") 2 3)"
            + "  (data (i32.const 0x1234) \"\\01\\02\\03\\04\")"
            + "  (func (export \"load\") (param $addr i32) (result i32)"
            + "    (i32.load8_s (local.get $addr))"
            + "  )"
            + "  (func (export \"store\") (param $addr i32) (param $val i32)"
            + "    (i32.store8 (local.get $addr) (local.get $val))"
            + "  )"
            + ')').getBytes();

    private Store<Void> store;
    private Engine engine;
    private Module module;
    private Instance instance;

    @Before
    public void setUp() {
        store = Store.withoutData();
        engine = store.engine();
        module = new Module(engine, WAT_BYTES);
        instance = new Instance(store, module, Collections.emptyList());
    }

    @After
    public void tearDown() {
        instance.close();
        module.close();
        engine.close();
        store.close();
    }

    @Test
    public void testSize() {
        try (Memory mem = instance.getMemory(store, "memory").get()) {
            assertEquals(PAGE_SIZE * 2, mem.dataSize(store));
            assertEquals(2, mem.size(store));
        }
    }

    @Test
    public void testExternalRead() {
        try (Memory mem = instance.getMemory(store, "memory").get()) {
            ByteBuffer buf = mem.buffer(store);
            assertEquals(1, buf.get(0x1234));
            assertEquals(3, buf.get(0x1234 + 2));
            try (Func storeFunc = instance.getFunc(store, "store").get()) {
                Consumer2<Integer, Integer> store = WasmFunctions.consumer(this.store, storeFunc, I32, I32);
                store.accept(0x1234 + 2, 10);
            }
            assertEquals(10, buf.get(0x1234 + 2));
        }
    }

    @Test
    public void testExternalWrite() {
        try (Memory mem = instance.getMemory(store, "memory").get()) {
            ByteBuffer buf = mem.buffer(store);
            try (Func loadFunc = instance.getFunc(store, "load").get()) {
                Function1<Integer, Integer> load = WasmFunctions.func(store, loadFunc, I32, I32);
                assertEquals(2, load.call(0x1234 + 1).intValue());

                buf.put(0x1234 + 1, (byte)10);
                assertEquals(10, load.call(0x1234 + 1).intValue());
            }
        }
    }

    @Test
    public void testGrow() {
        try (Memory mem = new Memory(store, new MemoryType(1024, false))) {
            long before = mem.size(store);
            mem.grow(store, 64);
            long after = mem.size(store);
            assertEquals(before + 64, after);
        }
    }
}
