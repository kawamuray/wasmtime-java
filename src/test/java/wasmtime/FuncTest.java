package wasmtime;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertNull;

import java.util.Arrays;
import java.util.Collections;
import java.util.Optional;

import org.junit.Test;

import wasmtime.Val.Type;

public class FuncTest {
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
    public void testCall() {
        try (Store store = new Store();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_ADD);
             Instance instance = new Instance(store, module, Collections.emptyList())) {
            try (Func func = instance.getFunc("add").get()) {
                Val[] results = func.call(Val.fromI32(1), Val.fromI32(2));
                assertEquals(1, results.length);
                assertEquals(Val.fromI32(3), results[0]);
            }
        }
    }

    @Test
    public void testTrampoline() {
        FuncType fnType = new FuncType(new Type[] { Type.I64, Type.I64 }, new Type[] { Type.I64 });
        try (Store store = new Store();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_TRAMPOLINE);
             Func callback = new Func(store, fnType,
                                      (caller, params, results) -> {
                                          results[0] = Val.fromI64(params[0].i64() + params[1].i64());
                                          return Optional.empty();
                                      });
             Instance instance = new Instance(store, module, Arrays.asList(Extern.fromFunc(callback)))) {
            try (Func func = instance.getFunc("trampoline").get()) {
                Val[] results = func.call(Val.fromI64(1), Val.fromI64(2));
                assertEquals(1, results.length);
                assertEquals(Val.fromI64(3), results[0]);
            }
        }
    }

    @Test(expected = WasmtimeException.class)
    public void testTrampolineTrap() {
        FuncType fnType = new FuncType(new Type[] { Type.I64, Type.I64 }, new Type[] { Type.I64 });
        try (Store store = new Store();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_TRAMPOLINE);
             Func callback = new Func(store, fnType,
                                      (caller, params, results) -> Optional.of(Trap.fromMessage("no hope...")));
             Instance instance = new Instance(store, module, Arrays.asList(Extern.fromFunc(callback)))) {
            try (Func func = instance.getFunc("trampoline").get()) {
                func.call(Val.fromI64(1), Val.fromI64(2));
            }
        }
    }

    @Test(expected = WasmtimeException.class)
    public void testTrampolineException() {
        FuncType fnType = new FuncType(new Type[] { Type.I64, Type.I64 }, new Type[] { Type.I64 });
        try (Store store = new Store();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_TRAMPOLINE);
             Func callback = new Func(store, fnType,
                                      (caller, params, results) -> {
                                          throw new RuntimeException("no hope...");
                                      });
             Instance instance = new Instance(store, module, Arrays.asList(Extern.fromFunc(callback)))) {
            try (Func func = instance.getFunc("trampoline").get()) {
                func.call(Val.fromI64(1), Val.fromI64(2));
            }
        }
    }

    @Test
    public void testTrampolineDrop() {
        FuncType fnType = new FuncType(new Type[] { Type.I64, Type.I64 }, new Type[] { Type.I64 });
        try (Store store = new Store()) {
            try (Func ignored = new Func(store, fnType, (caller, params, results) -> Optional.empty())) {
                assertEquals(1, Func.registry.map.size());
            }
        }
        assertEquals(0, Func.registry.map.size());
    }
}
