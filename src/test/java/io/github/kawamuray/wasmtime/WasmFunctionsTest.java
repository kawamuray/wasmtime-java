package io.github.kawamuray.wasmtime;

import static org.junit.Assert.assertEquals;
import static io.github.kawamuray.wasmtime.WasmValType.I32;
import static io.github.kawamuray.wasmtime.WasmValType.I64;

import java.util.Arrays;
import java.util.Collections;

import org.junit.Test;

import io.github.kawamuray.wasmtime.WasmFunctions.Function2;

public class WasmFunctionsTest {
    private static final byte[] WAT_BYTES_ADD = ("(module"
                                                 + "  (func (export \"add\") (param $p1 i32) (param $p2 i32) (result i32)"
                                                 + "    local.get $p1"
                                                 + "    local.get $p2"
                                                 + "    i32.add)"
                                                 + ')').getBytes();

    private static final byte[] WAT_BYTES_TRAMPOLINE = ("(module"
                                                        + "  (func $callback (import \"\" \"callback\") (param i64 i64) (result i64))"
                                                        + "  (func (export \"trampoline\") (param $p1 i64) (param $p2 i64) (result i64)"
                                                        + "    local.get $p1"
                                                        + "    local.get $p2"
                                                        + "    call $callback)"
                                                        + ')').getBytes();

    @Test
    public void testFunc() {
        try (Store<Void> store = Store.withoutData();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_ADD);
             Instance instance = new Instance(store, module, Collections.emptyList())) {
            try (Func func = instance.getFunc(store, "add").get()) {
                Function2<Integer, Integer, Integer> add = WasmFunctions.func(store, func, I32, I32, I32);
                assertEquals(3, add.call(1, 2).intValue());
            }
        }
    }

    @Test
    public void testWrapFunction() {
        try (Store<Void> store = Store.withoutData();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_TRAMPOLINE);
             Func callback = WasmFunctions.wrap(store, I64, I64, I64, (lhs, rhs) -> lhs + rhs);
             Instance instance = new Instance(store, module, Arrays.asList(Extern.fromFunc(callback)))) {
            try (Func func = instance.getFunc(store, "trampoline").get()) {
                Function2<Long, Long, Long> trampoline = WasmFunctions.func(store, func, I64, I64, I64);
                long sum = trampoline.call(1L, 2L);
                assertEquals(3L, sum);
            }
        }
    }
}
