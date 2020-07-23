#!/usr/bin/env python3

MAX_ARGS = 15

def types(i):
    return ", ".join(["A{0}".format(x) for x in range(i)])

def params(i):
    return ", ".join(["A{0} arg{0}".format(x) for x in range(i)])

def valtypes(i):
    return ", ".join(["ValType<A{0}> a{0}".format(x) for x in range(i)])

def type_calls(i):
    return ", ".join(["a{0}.type()".format(x) for x in range(i)])

def from_wasms(i):
    return ", ".join(["a{0}.fromWasmVal(params[{0}])".format(x) for x in range(i)])

def to_wasms(i):
    return ", ".join(["a{0}.toWasmVal(arg{0})".format(x) for x in range(i)])

print("""// Automatically generated. DO NOT EDIT.
package wasmtime;

import java.util.Arrays;
import java.util.Optional;

@SuppressWarnings("ALL")
public final class WasmFunctions {
    private static final Val.Type[] EMPTY_TYPES = new Val.Type[0];

    private WasmFunctions() {}
""");

# Function
for i in range(MAX_ARGS + 1):
    print("""    public interface Function{i}<{types}R0> {{
        R0 call({params});
    }}
""".format(i=i, types=types(i)+', ' if i > 0 else '', params=params(i)))

# Consumer
for i in range(MAX_ARGS + 1):
    print("""    public interface Consumer{i}{types} {{
        void accept({params});
    }}
""".format(i=i, types='<'+types(i)+'>' if i > 0 else '', params=params(i)))

# wrap for functions
for i in range(MAX_ARGS + 1):
    print("""    public static <{types}R0> Func wrap(Store store, {valtypes}ValType<R0> r0, Function{i}<{types}R0> func) {{
        return new Func(store, new FuncType(new Val.Type[] {{ {type_calls} }}, new Val.Type[] {{ r0.type() }}),
                        (caller, params, results) -> {{
                            R0 ret = func.call({from_wasms});
                            results[0] = r0.toWasmVal(ret);
                            return Optional.empty();
                        }});
    }}
""".format(i=i, types=types(i)+', ' if i > 0 else '', valtypes=valtypes(i)+', ' if i > 0 else '',
           type_calls=type_calls(i),
           from_wasms=from_wasms(i)))

# wrap for consumers
for i in range(MAX_ARGS + 1):
    print("""    public static {types} Func wrap(Store store, {valtypes}Consumer{i}{types} func) {{
        return new Func(store, new FuncType(new Val.Type[] {{ {type_calls} }}, EMPTY_TYPES),
                        (caller, params, results) -> {{
                            func.accept({from_wasms});
                            return Optional.empty();
                        }});
    }}
""".format(i=i, types='<'+types(i)+'>' if i > 0 else '', valtypes=valtypes(i)+', ' if i > 0 else '',
           type_calls=type_calls(i),
           from_wasms=from_wasms(i)))

# wrap for functions
for i in range(MAX_ARGS + 1):
    print("""    public static <{types}R0> Function{i}<{types}R0> func(Func fn, {valtypes}ValType<R0> r0) {{
        return ({args}) -> r0.fromWasmVal(fn.call(Arrays.asList({to_wasms})).get(0));
    }}
""".format(i=i, types=types(i)+', ' if i > 0 else '', valtypes=valtypes(i)+', ' if i > 0 else '',
           args=', '.join(["arg{0}".format(x) for x in range(i)]),
           to_wasms=to_wasms(i)))

# wrap for consumers
for i in range(MAX_ARGS + 1):
    print("""    public static {types} Consumer{i}{types} consumer(Func fn{valtypes}) {{
        return ({args}) -> fn.call(Arrays.asList({to_wasms}));
    }}
""".format(i=i, types='<'+types(i)+'>' if i > 0 else '', valtypes=', '+valtypes(i) if i > 0 else '',
           args=', '.join(["arg{0}".format(x) for x in range(i)]),
           to_wasms=to_wasms(i)))

print("}");
