package io.github.kawamuray.wasmtime;

import org.junit.Assert;
import org.junit.Test;

public class ModuleTest {
    private static final byte[] WAT_BINARY = ("(module"
                                             + "  (func (export \"add\") (param $p1 i32) (param $p2 i32) (result i32)"
                                             + "    local.get $p1\n"
                                             + "    local.get $p2\n"
                                             + "    i32.add)"
                                             + ")").getBytes();

    private static final byte[] IMPORT_WAT_BINARY = ("(module" +
                                                     "  (func $hello (import \"first\" \"package\"))\n" +
                                                     "  (func $world (import \"\" \"package\"))\n" +
                                                     "  (func (export \"run\") (call $hello) (call $world))\n" +
                                                     ")").getBytes();
    @Test
    public void testCreateDispose() {
        try (Engine engine = new Engine()) {
            Module module = new Module(engine, WAT_BINARY);
            module.close();
        }
    }

    @Test
    public void testAccessImports() {
        try (Engine engine = new Engine()) {
            Module module = new Module(engine, IMPORT_WAT_BINARY);
            String[][] names = {
                {"first", "package", "FUNC"},
                {"", "package", "FUNC"}
            };
            int i = 0;
            ImportType[] imports = module.imports();
            for (ImportType imp : imports) {
                Assert.assertEquals(names[i][0], imp.module());
                Assert.assertEquals(names[i][1], imp.name());
                Assert.assertEquals(names[i][2], imp.ty());
                imp.dispose();
                i+= 1;
            }
            module.close();
        }
    }
}
