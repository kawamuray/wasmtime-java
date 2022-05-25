package io.github.kawamuray.wasmtime;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;
import lombok.extern.slf4j.Slf4j;

import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.Optional;

/**
 * A WebAssembly function which can be called.
 *
 * This type is either provided by a WebAssembly Module or implemented in Java as a Host-function.
 *
 * A Func "belongs" to the store that it was originally created within. Operations on a Func only work with the store
 * it belongs to. Otherwise an exception will be thrown.
 * 
  * @see <a href="https://docs.wasmtime.dev/api/wasmtime/struct.Func.html">Rust Documentation</a>
  */
@Slf4j
@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Func implements Disposable {
    @FunctionalInterface
    public interface Handler<T> {
        Optional<Trap> call(Caller<T> caller, Val[] params, Val[] results);
    }

    private static final Val[] EMPTY_VALS = new Val[0];
    static final FuncRegistry registry = new FuncRegistry();

    @Getter(AccessLevel.PACKAGE)
    private final long innerPtr;

    public <T> Func(Store<T> store, FuncType fnType, Handler<T> func) {
        this(create(store, fnType, func));
    }

    private static <T> long create(Store<T> store, FuncType fnType, Handler<T> handler) {
        int index = registry.acquire(handler);
        log.debug("New trampoline {} of type {}", index, fnType);
        return newFunc(store.innerPtr(), fnType, index);
    }

    /**
     * Call this function with given variadic arguments
     * @param args a collection of argument values passed to the callee function
     * @return a list of returned values
     * @throws TrapException if the function throws an exception or exits with WASI API
     * @throws WasmtimeException if the wasmtime runtime throws an internal exception
     */
    public <T> Val[] call(Store<T> store, Val... args) {
        return nativeCall(store.innerPtr(), args);
    }

    /**
     * Call this function with a given list of arguments
     * @param args a collection of argument values passed to the callee function
     * @return a list of returned values
     * @throws TrapException if the function throws an exception or exits with WASI API
     * @throws WasmtimeException if the wasmtime runtime throws an internal exception
     */
    public <T> List<Val> call(Store<T> store, Collection<Val> args) {
        return Arrays.asList(call(store, args.toArray(EMPTY_VALS)));
    }

    // "trampoline" method to call back Java function from wasmtime through JNI code
    private static <T> Trap invokeTrampoline(long callerPtr, int index, Val[] params, Val[] results) {
        if (log.isDebugEnabled()) {
            log.debug("Trampoline {} invoked with params={}, results={}", index, params, results);
        }
        Handler<T> fn = registry.lookup(index);
        if (fn == null) {
            return Trap.fromMessage("no trampoline function associated to index: " + index);
        }
        try (Caller<T> caller = new Caller<>(callerPtr)) {
            return fn.call(caller, params, results).orElse(null);
        } catch (Throwable e) {
            return Trap.fromException(e);
        }
    }

    // The method called from JNI to drop Java function imported into wasm instance when it becomes unnecessary.
    private static void dropTrampoline(int index) {
        log.debug("Dropping trampoline {}", index);
        registry.drop(index);
    }

    @Override
    public native void dispose();

    private static native long newFunc(long storePtr, FuncType fnType, int index);

    private native Val[] nativeCall(long storePtr, Val[] args);
}
