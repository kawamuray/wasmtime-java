package io.github.kawamuray.wasmtime;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.fail;

import java.nio.charset.StandardCharsets;
import java.util.Collections;

import org.junit.Test;

import io.github.kawamuray.wasmtime.WasmFunctionError.TrapError;
import io.github.kawamuray.wasmtime.WasmFunctions.Consumer0;

public class TrapTest {
    private static final String INTERRUPT_WAT = "(module\n"
                                                + "  (func (export \"run\")\n"
                                                + "    (loop\n"
                                                + "      br 0)\n"
                                                + "  )\n"
                                                + ")\n";

    @Test
    public void interrupt() {
        try (Engine engine = new Engine(new Config().epochInterruption(true));
             Store<Void> store = new Store<>(null, engine)) {
            store.setEpochDeadline(1);

            // Compile and instantiate a small example with an infinite loop.
            try (Module module = new Module(engine, INTERRUPT_WAT.getBytes(StandardCharsets.UTF_8));
                 Instance instance = new Instance(store, module, Collections.emptyList());
                 Func runFn = instance.getFunc(store, "run").get()) {
                Consumer0 run = WasmFunctions.consumer(store, runFn);

                new Thread(() -> {
                    try {
                        Thread.sleep(1);
                    } catch (InterruptedException e) {
                        throw new RuntimeException(e);
                    }
                    engine.incrementEpoch();
                }).start();

                try {
                    run.accept();
                    fail("no trap received");
                } catch (TrapError e) {
                    assertEquals(Trap.INTERRUPT, e.trap());
                }
            }
        }
    }
}
