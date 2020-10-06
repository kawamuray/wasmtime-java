package io.github.kawamuray.wasmtime;

import static org.junit.Assert.assertEquals;
import static io.github.kawamuray.wasmtime.WasmValType.I32;

import java.nio.ByteBuffer;
import java.util.Collections;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

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

    private Store store;
    private Engine engine;
    private Module module;
    private Instance instance;

    @Before
    public void setUp() {
        store = new Store();
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
        try (Memory mem = instance.getMemory("memory").get()) {
            assertEquals(PAGE_SIZE * 2, mem.dataSize());
            assertEquals(2, mem.size());
        }
    }

    @Test
    public void testExternalRead() {
        try (Memory mem = instance.getMemory("memory").get()) {
            ByteBuffer buf = mem.buffer();
            assertEquals(1, buf.get(0x1234));
            assertEquals(3, buf.get(0x1234 + 2));
            try (Func storeFunc = instance.getFunc("store").get()) {
                Consumer2<Integer, Integer> store = WasmFunctions.consumer(storeFunc, I32, I32);
                store.accept(0x1234 + 2, 10);
            }
            assertEquals(10, buf.get(0x1234 + 2));
        }
    }

    @Test
    public void testExternalWrite() {
        try (Memory mem = instance.getMemory("memory").get()) {
            ByteBuffer buf = mem.buffer();
            try (Func loadFunc = instance.getFunc("load").get()) {
                Function1<Integer, Integer> load = WasmFunctions.func(loadFunc, I32, I32);
                assertEquals(2, load.call(0x1234 + 1).intValue());

                buf.put(0x1234 + 1, (byte) 10);
                assertEquals(10, load.call(0x1234 + 1).intValue());
            }
        }
    }
}
