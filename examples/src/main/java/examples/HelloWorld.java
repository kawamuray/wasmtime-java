package examples;

import java.util.Arrays;
import java.util.Collection;

import io.github.kawamuray.wasmtime.Engine;
import io.github.kawamuray.wasmtime.Extern;
import io.github.kawamuray.wasmtime.Func;
import io.github.kawamuray.wasmtime.Instance;
import io.github.kawamuray.wasmtime.Module;
import io.github.kawamuray.wasmtime.Store;
import io.github.kawamuray.wasmtime.WasmFunctions;

public class HelloWorld {
    public static void main(String[] args) {
        // Configure the initial compilation environment, creating the global
        // `Store` structure. Note that you can also tweak configuration settings
        // with a `Config` and an `Engine` if desired.
        System.err.println("Initializing...");
        try (Store<Void> store = Store.withoutData()) {
            // Compile the wasm binary into an in-memory instance of a `Module`.
            System.err.println("Compiling module...");
            try (Engine engine = store.engine();
                 Module module = Module.fromFile(engine, "./hello.wat")) {
                // Here we handle the imports of the module, which in this case is our
                // `HelloCallback` type and its associated implementation of `Callback.
                System.err.println("Creating callback...");
                try (Func helloFunc = WasmFunctions.wrap(store, () -> {
                    System.err.println("CB!! Calling back...");
                    System.err.println("CB!! > Hello World!");
                })) {
                    // Once we've got that all set up we can then move to the instantiation
                    // phase, pairing together a compiled module as well as a set of imports.
                    // Note that this is where the wasm `start` function, if any, would run.
                    System.err.println("Instantiating module...");
                    Collection<Extern> imports = Arrays.asList(Extern.fromFunc(helloFunc));
                    try (Instance instance = new Instance(store, module, imports)) {
                        // Next we poke around a bit to extract the `run` function from the module.
                        System.err.println("Extracting export...");
                        try (Func f = instance.getFunc(store, "run").get()) {
                            WasmFunctions.Consumer0 fn = WasmFunctions.consumer(store, f);

                            // And last but not least we can call it!
                            System.err.println("Calling export...");
                            fn.accept();

                            System.err.println("Done.");
                        }
                    }
                }
            }
        }
    }
}
