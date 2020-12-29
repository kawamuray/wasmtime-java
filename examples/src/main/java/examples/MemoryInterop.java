package examples;

import static io.github.kawamuray.wasmtime.WasmValType.I32;
import static io.github.kawamuray.wasmtime.WasmValType.I64;

import java.nio.ByteBuffer;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicReference;

import io.github.kawamuray.wasmtime.Extern;
import io.github.kawamuray.wasmtime.Func;
import io.github.kawamuray.wasmtime.Linker;
import io.github.kawamuray.wasmtime.Memory;
import io.github.kawamuray.wasmtime.Module;
import io.github.kawamuray.wasmtime.Store;
import io.github.kawamuray.wasmtime.WasmFunctions;
import io.github.kawamuray.wasmtime.WasmFunctions.Consumer0;
import io.github.kawamuray.wasmtime.wasi.Wasi;
import io.github.kawamuray.wasmtime.wasi.WasiConfig;
import io.github.kawamuray.wasmtime.wasi.WasiConfig.PreopenDir;

public class MemoryInterop {
    // Build it with `cargo wasi build`
    private static final String WASM_PATH = "./memory-interop/target/wasm32-wasi/debug/memory_interop.wasm";

    public static void main(String[] args) {
        String[] words = { "Hello", "from", "Java!" };
        AtomicInteger counter = new AtomicInteger();

        // Let the poll_word function to refer this as a placeholder of Memory because
        // we have to add the function as import before loading the module exporting Memory.
        AtomicReference<Memory> memRef = new AtomicReference<>();
        try (Store store = new Store();
             Linker linker = new Linker(store);
             Wasi wasi = new Wasi(store, new WasiConfig(new String[0], new PreopenDir[0]));
             Func pollWordFn = WasmFunctions.wrap(store, I64, I32, I32, (addr, len) -> {
                 System.err.println("Address to store word: " + addr);
                 ByteBuffer buf = memRef.get().buffer();
                 String word = words[counter.getAndIncrement() % words.length];
                 for (int i = 0; i < len && i < word.length(); i++) {
                     buf.put(addr.intValue() + i, (byte) word.charAt(i));
                 }
                 return Math.min(word.length(), len);
             });
             Module module = Module.fromFile(store.engine(), WASM_PATH)) {

            wasi.addToLinker(linker);
            linker.define("xyz", "poll_word", Extern.fromFunc(pollWordFn));
            linker.module("", module);

            try (Memory mem = linker.getOneByName("", "memory").memory();
                 Func doWorkFn = linker.getOneByName("", "do_work").func()) {
                memRef.set(mem);
                Consumer0 doWork = WasmFunctions.consumer(doWorkFn);
                doWork.accept();
                doWork.accept();
                doWork.accept();
            }
        }
    }
}
