package wasmtime;

import org.junit.Test;

public class ModuleTest {
    private static final byte[] WAT_BINARY = ("(module"
                                             + "  (func (export \"add\") (param $p1 i32) (param $p2 i32) (result i32)"
                                             + "    local.get $p1\n"
                                             + "    local.get $p2\n"
                                             + "    i32.add)"
                                             + ")").getBytes();
    @Test
    public void testCreateDispose() {
        try (Engine engine = new Engine()) {
            Module module = new Module(engine, WAT_BINARY);
            module.close();
        }
    }
}
