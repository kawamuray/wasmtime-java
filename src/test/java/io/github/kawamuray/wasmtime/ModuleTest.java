package io.github.kawamuray.wasmtime;

import lombok.Data;
import org.junit.Assert;
import org.junit.Test;

import java.util.function.Consumer;

public class ModuleTest {
    private static final byte[] WAT_BINARY = ("(module"
                                              + "  (func (export \"add\") (param $p1 i32) (param $p2 i32) (result i32)"
                                              + "    local.get $p1\n"
                                              + "    local.get $p2\n"
                                              + "    i32.add)"
                                              + ")").getBytes();

    // Required: Only one memory may be defined
    private static final byte[] MEM2 = ("(module" +
                                        "  (import \"mem\" \"two\" (memory $mem2 13 37))" +
                                        ")").getBytes();

    private static final byte[] IMPORT_WAT_BINARY = ("(module" +
                                                     "  (global $m1 (import \"globals\" \"mutable\") (mut i32))\n" +
                                                     "  (global $c2 (import \"globalz\" \"const\") i64)\n" +
                                                     "  (func $hello (import \"first\" \"package\"))\n" +
                                                     "  (import \"tbl\" \"small\" (table 0 4 anyfunc))\n" +
                                                     "  (import \"tbl\" \"big\" (table 12 1995 anyfunc))\n" +
                                                     "  (import \"lua\" \"integration\" (table 1 anyfunc))\n" +
                                                     "  (import \"env\" \"memory\" (memory $mem 1))\n" +
                                                     "  (import \"\" \"package\" (func $world (param $p1 i32)))\n" +
                                                     "  (import \"xyz\" \"return\" (func (result i32)))\n" +
                                                     "  (import \"xyz\" \"return\" (func (param i32 i32 i32 i32 i32)))\n" +
                                                     "  (func (export \"run\") (call $hello))\n" +
                                                     ")").getBytes();

    @Test
    public void testCreateDispose() {
        try (Engine engine = new Engine()) {
            Module module = new Module(engine, WAT_BINARY);
            module.close();
        }
    }

    @Test
    public void testMem2() {
        try (
            Engine engine = new Engine();
            Module module = new Module(engine, MEM2)
        ) {
            runImportTest(module, new TestImportData[]{
                TestImportData.memory("mem", "two", 13, 37)
            });
        }
    }

    @Test
    public void testAccessImports() {
        try (
            Engine engine = new Engine();
            Module module = new Module(engine, IMPORT_WAT_BINARY)
        ) {
            // TODO: Test other import typese
            runImportTest(module, new TestImportData[]{
                TestImportData.global("globals", "mutable", Val.Type.I32, Mutability.VAR),
                TestImportData.global("globalz", "const", Val.Type.I64, Mutability.CONST),
                TestImportData.func("first", "package", new Val.Type[]{}, new Val.Type[]{}),
                TestImportData.table("tbl", "small", Val.Type.FUNC_REF, 0, 4),
                TestImportData.table("tbl", "big", Val.Type.FUNC_REF, 12, 1995),
                TestImportData.table("lua", "integration", Val.Type.FUNC_REF, 1, -1),
                TestImportData.memory("env", "memory", 1, -1),
                TestImportData.func("", "package", new Val.Type[]{Val.Type.I32}, new Val.Type[]{}),
                TestImportData.func("xyz", "return", new Val.Type[]{}, new Val.Type[]{Val.Type.I32}),
                TestImportData.func("xyz", "return", new Val.Type[]{Val.Type.I32, Val.Type.I32, Val.Type.I32, Val.Type.I32, Val.Type.I32}, new Val.Type[]{})
            });
        }
    }

    private void runImportTest(Module module, TestImportData<?>[] testData) {
        int i = 0;
        for (ImportType imp : module.imports()) {
            Assert.assertTrue("Test Data not big enough", testData.length > i);
            TestImportData<?> data = testData[i];
            Assert.assertEquals(data.getModule(), imp.module());
            Assert.assertEquals(data.getName(), imp.name());
            Assert.assertEquals(data.getType(), imp.type());
            checkImportType(data, imp);
            i += 1;
        }
        Assert.assertEquals("Not Every Test Case was returned", testData.length, i);
    }

    private <T> void checkImportType(TestImportData<T> data, ImportType type) {
        Class<T> clazz = data.getClazz();
        Object typeObj = type.typeObj();
        Assert.assertNotNull("Type Object is null", typeObj);
        Class<?> typeObjClass = typeObj.getClass();
        Assert.assertTrue(String.format("Expected Type is different. Expected %s but was %s", clazz, typeObjClass), clazz.isAssignableFrom(typeObjClass));
        data.verify(type, typeObj);
    }

    @Data
    private static class TestImportData<T> {
        private final String module;
        private final String name;
        private final ImportType.Type type;
        private final Class<T> clazz;
        private final Consumer<ImportType> verifyImport;
        private final Consumer<T> consumer;

        static TestImportData<MemoryType> memory(String module, String name, int min, int max) {
            return new TestImportData<>(
                module, name, ImportType.Type.MEMORY, MemoryType.class,
                mod -> {
                    Assert.assertThrows(RuntimeException.class, mod::func);
                    Assert.assertThrows(RuntimeException.class, mod::global);
                    Assert.assertEquals(mod.typeObj(), mod.memory());
                    Assert.assertThrows(RuntimeException.class, mod::table);
                },
                mem -> {
                    MemoryType.Limit limit = mem.limit();
                    Assert.assertEquals(min, limit.min());
                    Assert.assertEquals(max, limit.max());
                }
            );
        }

        static TestImportData<GlobalType> global(String module, String name, Val.Type content, Mutability mutability) {
            return new TestImportData<>(
                module, name, ImportType.Type.GLOBAL, GlobalType.class,
                mod -> {
                    Assert.assertThrows(RuntimeException.class, mod::func);
                    Assert.assertEquals(mod.typeObj(), mod.global());
                    Assert.assertThrows(RuntimeException.class, mod::memory);
                    Assert.assertThrows(RuntimeException.class, mod::table);
                },
                global -> {
                    Assert.assertEquals(content, global.getContent());
                    Assert.assertEquals(mutability, global.getMutability());
                }
            );
        }

        static TestImportData<FuncType> func(String module, String name, Val.Type[] params, Val.Type[] results) {
            return new TestImportData<>(
                module, name, ImportType.Type.FUNC, FuncType.class,
                mod -> {
                    Assert.assertEquals(mod.typeObj(), mod.func());
                    Assert.assertThrows(RuntimeException.class, mod::global);
                    Assert.assertThrows(RuntimeException.class, mod::memory);
                    Assert.assertThrows(RuntimeException.class, mod::table);
                },
                func -> {
                    Assert.assertArrayEquals(params, func.getParams());
                    Assert.assertArrayEquals(results, func.getResults());
                }
            );
        }

        public static TestImportData<TableType> table(String module, String name, Val.Type content, int min, int max) {
            return new TestImportData<>(
                module, name, ImportType.Type.TABLE, TableType.class,
                mod -> {
                    Assert.assertThrows(RuntimeException.class, mod::func);
                    Assert.assertThrows(RuntimeException.class, mod::global);
                    Assert.assertThrows(RuntimeException.class, mod::memory);
                    Assert.assertEquals(mod.typeObj(), mod.table());
                },
                table -> {
                    Assert.assertEquals(content, table.element());

                    MemoryType.Limit limit = table.limit();
                    Assert.assertEquals(min, limit.min());
                    Assert.assertEquals(max, limit.max());
                }
            );
        }

        @SuppressWarnings("unchecked")
        public void verify(final ImportType imp, final Object typeObj) {
            this.verifyImport.accept(imp);
            this.consumer.accept((T) typeObj);
        }
    }
}
