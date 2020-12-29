package examples;

import static io.github.kawamuray.wasmtime.WasmValType.I32;
import static java.util.Collections.emptyList;

import io.github.kawamuray.wasmtime.Func;
import io.github.kawamuray.wasmtime.Instance;
import io.github.kawamuray.wasmtime.Module;
import io.github.kawamuray.wasmtime.Store;
import io.github.kawamuray.wasmtime.WasmFunctions;

public class Gcd {
	public static void main(String[] args) {
		// Configure the initial compilation environment, creating the global
		// `Store` structure. jaNote that you can also tweak configuration settings
		// with a `Config` and an `Engine` if desired.
		System.err.println("Initializing...");
		try (Store store = new Store();
			 Module module = Module.fromFile(store.engine(), "./gcd.wat");
			 Instance instance = new Instance(store, module, emptyList());
			 Func fun = instance.getFunc("gcd").get()) {
			WasmFunctions.Function2<Integer, Integer, Integer> gcd = WasmFunctions.func(fun, I32, I32, I32);
			System.err.println("result = " + gcd.call(6, 27));
			System.err.println("Done.");
		}
	}
}
