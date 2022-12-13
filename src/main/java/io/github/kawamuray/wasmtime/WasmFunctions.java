// Automatically generated. DO NOT EDIT.
package io.github.kawamuray.wasmtime;

import java.util.Arrays;

@SuppressWarnings("ALL")
public final class WasmFunctions {
    private static final Val.Type[] EMPTY_TYPES = new Val.Type[0];

    private WasmFunctions() {}

    public interface Function0<R0> {
        R0 call();
    }

    public interface Function1<A0, R0> {
        R0 call(A0 arg0);
    }

    public interface Function2<A0, A1, R0> {
        R0 call(A0 arg0, A1 arg1);
    }

    public interface Function3<A0, A1, A2, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2);
    }

    public interface Function4<A0, A1, A2, A3, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3);
    }

    public interface Function5<A0, A1, A2, A3, A4, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4);
    }

    public interface Function6<A0, A1, A2, A3, A4, A5, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5);
    }

    public interface Function7<A0, A1, A2, A3, A4, A5, A6, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6);
    }

    public interface Function8<A0, A1, A2, A3, A4, A5, A6, A7, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7);
    }

    public interface Function9<A0, A1, A2, A3, A4, A5, A6, A7, A8, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8);
    }

    public interface Function10<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9);
    }

    public interface Function11<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10);
    }

    public interface Function12<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10, A11 arg11);
    }

    public interface Function13<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10, A11 arg11, A12 arg12);
    }

    public interface Function14<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10, A11 arg11, A12 arg12, A13 arg13);
    }

    public interface Function15<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, R0> {
        R0 call(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10, A11 arg11, A12 arg12, A13 arg13, A14 arg14);
    }

    public interface Consumer0 {
        void accept();
    }

    public interface Consumer1<A0> {
        void accept(A0 arg0);
    }

    public interface Consumer2<A0, A1> {
        void accept(A0 arg0, A1 arg1);
    }

    public interface Consumer3<A0, A1, A2> {
        void accept(A0 arg0, A1 arg1, A2 arg2);
    }

    public interface Consumer4<A0, A1, A2, A3> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3);
    }

    public interface Consumer5<A0, A1, A2, A3, A4> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4);
    }

    public interface Consumer6<A0, A1, A2, A3, A4, A5> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5);
    }

    public interface Consumer7<A0, A1, A2, A3, A4, A5, A6> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6);
    }

    public interface Consumer8<A0, A1, A2, A3, A4, A5, A6, A7> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7);
    }

    public interface Consumer9<A0, A1, A2, A3, A4, A5, A6, A7, A8> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8);
    }

    public interface Consumer10<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9);
    }

    public interface Consumer11<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10);
    }

    public interface Consumer12<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10, A11 arg11);
    }

    public interface Consumer13<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10, A11 arg11, A12 arg12);
    }

    public interface Consumer14<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10, A11 arg11, A12 arg12, A13 arg13);
    }

    public interface Consumer15<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14> {
        void accept(A0 arg0, A1 arg1, A2 arg2, A3 arg3, A4 arg4, A5 arg5, A6 arg6, A7 arg7, A8 arg8, A9 arg9, A10 arg10, A11 arg11, A12 arg12, A13 arg13, A14 arg14);
    }

    public static <D, R0> Func wrap(Store<D> store, WasmValType<R0> r0, Function0<R0> func) {
        return new Func(store, new FuncType(new Val.Type[] {  }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call();
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<R0> r0, Function1<A0, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<R0> r0, Function2<A0, A1, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<R0> r0, Function3<A0, A1, A2, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<R0> r0, Function4<A0, A1, A2, A3, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<R0> r0, Function5<A0, A1, A2, A3, A4, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<R0> r0, Function6<A0, A1, A2, A3, A4, A5, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<R0> r0, Function7<A0, A1, A2, A3, A4, A5, A6, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<R0> r0, Function8<A0, A1, A2, A3, A4, A5, A6, A7, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<R0> r0, Function9<A0, A1, A2, A3, A4, A5, A6, A7, A8, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<R0> r0, Function10<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<R0> r0, Function11<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<R0> r0, Function12<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type(), a11.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]), a11.fromWasmVal(params[11]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<R0> r0, Function13<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type(), a11.type(), a12.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]), a11.fromWasmVal(params[11]), a12.fromWasmVal(params[12]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<A13> a13, WasmValType<R0> r0, Function14<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type(), a11.type(), a12.type(), a13.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]), a11.fromWasmVal(params[11]), a12.fromWasmVal(params[12]), a13.fromWasmVal(params[13]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, R0> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<A13> a13, WasmValType<A14> a14, WasmValType<R0> r0, Function15<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, R0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type(), a11.type(), a12.type(), a13.type(), a14.type() }, new Val.Type[] { r0.type() }),
                        (caller, params, results) -> {
                            R0 ret = func.call(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]), a11.fromWasmVal(params[11]), a12.fromWasmVal(params[12]), a13.fromWasmVal(params[13]), a14.fromWasmVal(params[14]));
                            results[0] = r0.toWasmVal(ret);
                        });
    }

    public static <D> Func wrap(Store<D> store, Consumer0 func) {
        return new Func(store, new FuncType(new Val.Type[] {  }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept();
                        });
    }

    public static <D,A0> Func wrap(Store<D> store, WasmValType<A0> a0, Consumer1<A0> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]));
                        });
    }

    public static <D,A0, A1> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, Consumer2<A0, A1> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]));
                        });
    }

    public static <D,A0, A1, A2> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, Consumer3<A0, A1, A2> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]));
                        });
    }

    public static <D,A0, A1, A2, A3> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, Consumer4<A0, A1, A2, A3> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, Consumer5<A0, A1, A2, A3, A4> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, Consumer6<A0, A1, A2, A3, A4, A5> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5, A6> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, Consumer7<A0, A1, A2, A3, A4, A5, A6> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5, A6, A7> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, Consumer8<A0, A1, A2, A3, A4, A5, A6, A7> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5, A6, A7, A8> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, Consumer9<A0, A1, A2, A3, A4, A5, A6, A7, A8> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5, A6, A7, A8, A9> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, Consumer10<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, Consumer11<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, Consumer12<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type(), a11.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]), a11.fromWasmVal(params[11]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, Consumer13<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type(), a11.type(), a12.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]), a11.fromWasmVal(params[11]), a12.fromWasmVal(params[12]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<A13> a13, Consumer14<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type(), a11.type(), a12.type(), a13.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]), a11.fromWasmVal(params[11]), a12.fromWasmVal(params[12]), a13.fromWasmVal(params[13]));
                        });
    }

    public static <D,A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14> Func wrap(Store<D> store, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<A13> a13, WasmValType<A14> a14, Consumer15<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14> func) {
        return new Func(store, new FuncType(new Val.Type[] { a0.type(), a1.type(), a2.type(), a3.type(), a4.type(), a5.type(), a6.type(), a7.type(), a8.type(), a9.type(), a10.type(), a11.type(), a12.type(), a13.type(), a14.type() }, EMPTY_TYPES),
                        (caller, params, results) -> {
                            func.accept(a0.fromWasmVal(params[0]), a1.fromWasmVal(params[1]), a2.fromWasmVal(params[2]), a3.fromWasmVal(params[3]), a4.fromWasmVal(params[4]), a5.fromWasmVal(params[5]), a6.fromWasmVal(params[6]), a7.fromWasmVal(params[7]), a8.fromWasmVal(params[8]), a9.fromWasmVal(params[9]), a10.fromWasmVal(params[10]), a11.fromWasmVal(params[11]), a12.fromWasmVal(params[12]), a13.fromWasmVal(params[13]), a14.fromWasmVal(params[14]));
                        });
    }

    public static <D, R0> Function0<R0> func(Store<D> store, Func fn, WasmValType<R0> r0) {
        return () -> r0.fromWasmVal(fn.call(store, Arrays.asList()).get(0));
    }

    public static <D, A0, R0> Function1<A0, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<R0> r0) {
        return (arg0) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0))).get(0));
    }

    public static <D, A0, A1, R0> Function2<A0, A1, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<R0> r0) {
        return (arg0, arg1) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1))).get(0));
    }

    public static <D, A0, A1, A2, R0> Function3<A0, A1, A2, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<R0> r0) {
        return (arg0, arg1, arg2) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2))).get(0));
    }

    public static <D, A0, A1, A2, A3, R0> Function4<A0, A1, A2, A3, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, R0> Function5<A0, A1, A2, A3, A4, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, R0> Function6<A0, A1, A2, A3, A4, A5, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, R0> Function7<A0, A1, A2, A3, A4, A5, A6, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, R0> Function8<A0, A1, A2, A3, A4, A5, A6, A7, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, R0> Function9<A0, A1, A2, A3, A4, A5, A6, A7, A8, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R0> Function10<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R0> Function11<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, R0> Function12<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10), a11.toWasmVal(arg11))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, R0> Function13<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10), a11.toWasmVal(arg11), a12.toWasmVal(arg12))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, R0> Function14<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<A13> a13, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10), a11.toWasmVal(arg11), a12.toWasmVal(arg12), a13.toWasmVal(arg13))).get(0));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, R0> Function15<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, R0> func(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<A13> a13, WasmValType<A14> a14, WasmValType<R0> r0) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14) -> r0.fromWasmVal(fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10), a11.toWasmVal(arg11), a12.toWasmVal(arg12), a13.toWasmVal(arg13), a14.toWasmVal(arg14))).get(0));
    }

    public static <D> Consumer0 consumer(Store<D> store, Func fn) {
        return () -> fn.call(store, Arrays.asList());
    }

    public static <D, A0> Consumer1<A0> consumer(Store<D> store, Func fn, WasmValType<A0> a0) {
        return (arg0) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0)));
    }

    public static <D, A0, A1> Consumer2<A0, A1> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1) {
        return (arg0, arg1) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1)));
    }

    public static <D, A0, A1, A2> Consumer3<A0, A1, A2> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2) {
        return (arg0, arg1, arg2) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2)));
    }

    public static <D, A0, A1, A2, A3> Consumer4<A0, A1, A2, A3> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3) {
        return (arg0, arg1, arg2, arg3) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3)));
    }

    public static <D, A0, A1, A2, A3, A4> Consumer5<A0, A1, A2, A3, A4> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4) {
        return (arg0, arg1, arg2, arg3, arg4) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4)));
    }

    public static <D, A0, A1, A2, A3, A4, A5> Consumer6<A0, A1, A2, A3, A4, A5> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5) {
        return (arg0, arg1, arg2, arg3, arg4, arg5) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5)));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6> Consumer7<A0, A1, A2, A3, A4, A5, A6> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6)));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7> Consumer8<A0, A1, A2, A3, A4, A5, A6, A7> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7)));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8> Consumer9<A0, A1, A2, A3, A4, A5, A6, A7, A8> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8)));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9> Consumer10<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9)));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> Consumer11<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10)));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11> Consumer12<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10), a11.toWasmVal(arg11)));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12> Consumer13<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10), a11.toWasmVal(arg11), a12.toWasmVal(arg12)));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13> Consumer14<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<A13> a13) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10), a11.toWasmVal(arg11), a12.toWasmVal(arg12), a13.toWasmVal(arg13)));
    }

    public static <D, A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14> Consumer15<A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14> consumer(Store<D> store, Func fn, WasmValType<A0> a0, WasmValType<A1> a1, WasmValType<A2> a2, WasmValType<A3> a3, WasmValType<A4> a4, WasmValType<A5> a5, WasmValType<A6> a6, WasmValType<A7> a7, WasmValType<A8> a8, WasmValType<A9> a9, WasmValType<A10> a10, WasmValType<A11> a11, WasmValType<A12> a12, WasmValType<A13> a13, WasmValType<A14> a14) {
        return (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13, arg14) -> fn.call(store, Arrays.asList(a0.toWasmVal(arg0), a1.toWasmVal(arg1), a2.toWasmVal(arg2), a3.toWasmVal(arg3), a4.toWasmVal(arg4), a5.toWasmVal(arg5), a6.toWasmVal(arg6), a7.toWasmVal(arg7), a8.toWasmVal(arg8), a9.toWasmVal(arg9), a10.toWasmVal(arg10), a11.toWasmVal(arg11), a12.toWasmVal(arg12), a13.toWasmVal(arg13), a14.toWasmVal(arg14)));
    }

}
