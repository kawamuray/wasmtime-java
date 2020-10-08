wasmtime-java
=============

Java (or any JVM) language binding for [Wasmtime](https://github.com/bytecodealliance/wasmtime).

Some basic examples are working, but many API implementations are work in progress.

# Declaring Dependencies

Gradle example:

```groovy
repositories {
    mavenCentral()
}

dependencies {
    implementation "io.github.kawamuray.wasmtime:wasmtime-java:$LATEST_VERSION"
}
```

# Example

See [examples](./examples) for the full example.

```java
public class HelloWasm {
    public static void main(String[] args) {
        try (Store store = new Store();
             Engine engine = store.engine();
             Module module = Module.fromFile(engine, "./hello.wat");
             Func helloFunc = WasmFunctions.wrap(store, () -> {
                 System.err.println(">>> Calling back...");
                 System.err.println(">>> Hello World!");
             })) {
            Collection<Extern> imports = Arrays.asList(Extern.fromFunc(helloFunc));
            try (Instance instance = new Instance(store, module, imports)) {
                try (Func f = instance.getFunc("run").get()) {
                    WasmFunctions.Consumer0 fn = WasmFunctions.consumer(f);
                    fn.accept();
                }
            }
        }
    }
}
```

Run example:
```sh
$ ./gradlew -Pmain=examples.HelloWorld examples:run
```

# How to build

```sh
$ ./gradlew build
```

# License

Apache License Version 2.0
