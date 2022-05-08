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
 * This type can represent either an exported function from a WebAssembly module or a host-defined function which can
 * be used to satisfy an import of a module. Func and can be used to both instantiate an Instance as well as be
 * extracted from an Instance.
 *
 * A Func "belongs" to the store that it was originally created within. Operations on a Func only work with the store
 * it belongs to, and if another store is passed in by accident then methods will panic.
 *
 * <h1>Func and async</h1>
 * Functions from the perspective of WebAssembly are always synchronous. You might have an async function in Rust,
 * however, which you’d like to make available from WebAssembly. Wasmtime supports asynchronously calling WebAssembly
 * through native stack switching. You can get some more information about asynchronous configs, but from the
 * perspective of Func it’s important to know that whether or not your Store is asynchronous will dictate whether you
 * call functions through Func::call or Func::call_async (or the typed wrappers such as TypedFunc::call vs
 * TypedFunc::call_async).
 *
 * <h1>To Func::call or to Func::typed().call()</h1>
 * There’s a 2x2 matrix of methods to call Func. Invocations can either be asynchronous or synchronous.
 * They can also be statically typed or not. Whether or not an invocation is asynchronous is indicated via the method
 * being async and call_async being the entry point. Otherwise for statically typed or not your options are:
 *
 *     Dynamically typed - if you don’t statically know the signature of the function that you’re calling you’ll be
 *          using Func::call or Func::call_async. These functions take a variable-length slice of "boxed" arguments
 *          in their Val representation. Additionally the results are returned as an owned slice of Val. These methods
 *          are not optimized due to the dynamic type checks that must occur, in addition to some dynamic allocations
 *          for where to put all the arguments. While this allows you to call all possible wasm function signatures,
 *          if you’re looking for a speedier alternative you can also use…
 *
 *     Statically typed - if you statically know the type signature of the wasm function you’re calling, then you’ll
 *          want to use the Func::typed method to acquire an instance of TypedFunc. This structure is static proof
 *          that the underlying wasm function has the ascripted type, and type validation is only done once up-front.
 *          The TypedFunc::call and TypedFunc::call_async methods are much more efficient than Func::call and
 *          Func::call_async because the type signature is statically known. This eschews runtime checks as much as
 *          possible to get into wasm as fast as possible.
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
