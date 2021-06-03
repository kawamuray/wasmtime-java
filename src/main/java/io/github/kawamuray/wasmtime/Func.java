package io.github.kawamuray.wasmtime;

import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.Optional;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.experimental.Accessors;
import lombok.extern.slf4j.Slf4j;

@Slf4j
@Accessors(fluent = true)
@AllArgsConstructor(access = AccessLevel.PACKAGE)
public class Func implements Disposable {
    @FunctionalInterface
    public interface Handler {
        Optional<Trap> call(Caller caller, Val[] params, Val[] results);
    }

    private static final Val[] EMPTY_VALS = new Val[0];
    static final FuncRegistry registry = new FuncRegistry();

    @Getter(AccessLevel.PACKAGE)
    private final long innerPtr;

    public Func(Store store, FuncType fnType, Handler func) {
        this(create(store, fnType, func));
    }

    private static long create(Store store, FuncType fnType, Handler handler) {
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
    public Val[] call(Val... args) {
        return nativeCall(args);
    }

    /**
     * Call this function with a given list of arguments
     * @param args a collection of argument values passed to the callee function
     * @return a list of returned values
     * @throws TrapException if the function throws an exception or exits with WASI API
     * @throws WasmtimeException if the wasmtime runtime throws an internal exception
     */
    public List<Val> call(Collection<Val> args) {
        return Arrays.asList(call(args.toArray(EMPTY_VALS)));
    }

    // "trampoline" method to call back Java function from wasmtime through JNI code
    private static Trap invokeTrampoline(long callerPtr, int index, Val[] params, Val[] results) {
        if (log.isDebugEnabled()) {
            log.debug("Trampoline {} invoked with params={}, results={}", index, params, results);
        }
        Handler fn = registry.lookup(index);
        if (fn == null) {
            return Trap.fromMessage("no trampoline function associated to index: " + index);
        }
        try (Caller caller = new Caller(callerPtr)) {
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

    private native Val[] nativeCall(Val[] args);
}
