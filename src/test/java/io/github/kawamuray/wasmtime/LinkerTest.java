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

    private static final byte[] WAT_BYTES_TABLE = ("(module"
            + "  (table (;0;) 8 8 funcref)(export \"__indirect_function_table\" (table 0))"
            + ')').getBytes();

    private static final byte[] WAT_BYTES_GLOBAL = ("(module"
            + "  (global (export \"global\") (mut i32) (i32.const 256))"
            + "  (table (export \"__indirect_function_table\") 8 8 funcref)"
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

    @Test
    public void testTableType() {
        try (Store<Void> store = Store.withoutData();
                Linker linker = new Linker(store.engine());
                Engine engine = store.engine();
                Module module = new Module(engine, WAT_BYTES_TABLE)) {
            linker.module(store, "", module);
            Collection<ExternItem> externs = linker.externs(store);
            assertEquals(1, externs.size());
            ExternItem item = externs.iterator().next();
            assertEquals("__indirect_function_table", item.name());
            assertEquals(Extern.Type.TABLE, item.extern().type());
        }
    }

    @Test
    public void testGlobalType() {
        try (Store<Void> store = Store.withoutData();
                Linker linker = new Linker(store.engine());
                Engine engine = store.engine();
                Module module = new Module(engine, WAT_BYTES_GLOBAL)) {
            linker.module(store, "", module);
            Collection<ExternItem> externs = linker.externs(store);
            assertEquals(2, externs.size());
            ExternItem item = externs.iterator().next();
            assertEquals(Extern.Type.GLOBAL, item.extern().type());
            assertEquals("global", item.name());
        }
    }

    @Test
    public void testExternsOfMulti() {
        try (Store<Void> store = Store.withoutData();
                Linker linker = new Linker(store.engine());
                Engine engine = store.engine();
                Module module = new Module(engine, WAT_BYTES_ADD)) {
            linker.module(store, "", module);
            linker.module(store, "other", new Module(engine, WAT_BYTES_TABLE));
            Collection<ExternItem> externs = linker.externsOfModule(store, "");
            assertEquals(1, externs.size());
            assertEquals("add", externs.iterator().next().name());
        }
    }

}
