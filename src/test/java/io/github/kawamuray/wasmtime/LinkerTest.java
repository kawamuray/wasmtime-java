package io.github.kawamuray.wasmtime;

import static org.junit.Assert.assertEquals;

import java.util.Collection;

import org.junit.Test;

public class LinkerTest {
    private static final byte[] WAT_BYTES_ADD = ("(module"
                                                 + "  (func (export \"add\") (param $p1 i32) (param $p2 i32) (result i32)"
                                                 + "    local.get $p1"
                                                 + "    local.get $p2"
                                                 + "    i32.add)"
                                                 + ')').getBytes();

    @Test
    public void testModules() {
        try (Store<Void> store = Store.withoutData();
             Linker linker = new Linker(store.engine());
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_ADD)) {
            linker.module(store, "", module);
            assertEquals(1, linker.modules(store).size());
            assertEquals("", linker.modules(store).iterator().next());
        }
    }

    @Test
    public void testExterns() {
        try (Store<Void> store = Store.withoutData();
             Linker linker = new Linker(store.engine());
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_ADD)) {
            linker.module(store, "", module);
            Collection<ExternItem> externs = linker.externs(store);
            assertEquals(1, externs.size());
            assertEquals("add", externs.iterator().next().name());
        }
    }

    @Test
    public void testExternsOfModule() {
        try (Store<Void> store = Store.withoutData();
             Linker linker = new Linker(store.engine());
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_ADD)) {
            linker.module(store, "", module);
            Collection<ExternItem> externs = linker.externsOfModule(store, "");
            assertEquals(1, externs.size());
            assertEquals("add", externs.iterator().next().name());
        }
    }
}
