package com.blocklynukkit.wasm.tests;

import io.github.kawamuray.wasmtime.*;
import io.github.kawamuray.wasmtime.wasi.Wasi;
import io.github.kawamuray.wasmtime.wasi.WasiConfig;

import java.util.Arrays;
import java.util.Collection;
import java.util.Collections;
import java.util.Random;

import static io.github.kawamuray.wasmtime.WasmValType.I32;
import static io.github.kawamuray.wasmtime.WasmValType.I64;

public class MoudleTest {
    static Random random = new Random(System.currentTimeMillis());
    public static void main(String[] args){
        for (int i = 0; i < 1; i++) {
            long start = System.currentTimeMillis();
            testWasmPrintDouble();
            System.out.println("总耗时:"+(System.currentTimeMillis()-start)+"ms");
        }
    }
    /**
     * @status Failed
     */
    public static void testEmscripten(){
        try(Store store = new Store();
            Engine engine = store.engine();
            Module module = Module.fromFile(engine,"emscripten.wasm");
            Linker linker = new Linker(store)){
            Func log = WasmFunctions.wrap(store,I32,I32,(arg) -> {
                System.out.println("回调完毕！"+arg);
                return arg+1;
            });
            Wasi wasi = new Wasi(store,new WasiConfig(new String[]{"-lc-printscan-long-double"},new WasiConfig.PreopenDir[0]));
            wasi.addToLinker(linker);
            linker.define("env","logInt",Extern.fromFunc(log));
            linker.module("test",module);
            linker.getOneByName("test","_start").func().call();
        }
    }
    public static void testWasmPrintDouble(){
        try(Store store = new Store();
            Engine engine = store.engine();
            Module module = Module.fromFile(engine,"printDouble.wasm");
            Linker linker = new Linker(store)){
            Func log = WasmFunctions.wrap(store,I32,I32,(arg) -> {
                System.out.println("回调完毕！"+arg);
                return arg+1;
            });
            Wasi wasi = new Wasi(store,new WasiConfig(new String[]{"-lc-printscan-long-double"},new WasiConfig.PreopenDir[0]));
            wasi.addToLinker(linker);
            linker.define("env","logInt",Extern.fromFunc(log));
            linker.module("test",module);
            linker.getOneByName("test","_start").func().call();
        }
    }
    public static void testLog(){
        byte[] WAT_BINARY = ("(module\n" +
                "  (import \"blocklynukkit\" \"logger_info\" (func $log (param i32) (result i32)))\n" +
                //"  (func $log (import \"\" \"log\" ) (param i32) (result i32))\n" +
                "  (func (export \"logIt\") (result i32)" +
                "    i32.const 88" +
                "    call $log)\n" +
                ")").getBytes();
        try(Store store = new Store();
            Engine engine = store.engine();
            Module module = new Module(engine, WAT_BINARY);
            Linker linker = new Linker(store)
            ){
            Wasi wasi = new Wasi(store,new WasiConfig(new String[0],new WasiConfig.PreopenDir[0]));
            Func log = WasmFunctions.wrap(store,I32,I32,(arg) -> {
                System.out.println("回调完毕！"+arg);
                return arg+1;
            });
            linker.define("blocklynukkit","logger_info",Extern.fromFunc(log));
            linker.module("test",module);
            wasi.addToLinker(linker);
            System.out.println("java获取"+linker.getOneByName("test","logIt").func().call()[0].i32());
        }
    }
    public static void testHello(){
        // Configure the initial compilation environment, creating the global
        // `Store` structure. Note that you can also tweak configuration settings
        // with a `Config` and an `Engine` if desired.
        byte[] WAT_HELLO = ("(module\n" +
                "  (func $hello (import \"\" \"hello\"))\n" +
                "  (func (export \"run\") (call $hello))\n" +
                ")").getBytes();
        System.err.println("Initializing...");
        try (Store store = new Store()) {
            System.err.println("Compiling module...");
            try (Engine engine = store.engine();
                 Module module = new Module(engine, WAT_HELLO)) {
                System.err.println("Creating callback...");
                try (Func helloFunc = WasmFunctions.wrap(store, () -> {
                    System.err.println("CB!! Calling back...");
                    System.err.println("CB!! > Hello World!");
                })) {
                    System.err.println("Instantiating module...");
                    Collection<Extern> imports = Arrays.asList(Extern.fromFunc(helloFunc));
                    try (Instance instance = new Instance(store, module, imports)) {
                        System.err.println("Extracting export...");
                        try (Func f = instance.getFunc("run").get()) {
                            WasmFunctions.Consumer0 fn = WasmFunctions.consumer(f);
                            System.err.println("Calling export...");
                            fn.accept();
                            System.err.println("Done.");
                        }
                    }
                }
            }
        }
    }
    public static void testMoudle(){
        byte[] WAT_BINARY = ("(module"
                + "  (func (export \"add\") (param $p1 i32) (param $p2 i32) (result i32)"
                + "    local.get $p1\n"
                + "    local.get $p2\n"
                + "    i32.add)"
                + ")").getBytes();
        try (Engine engine = new Engine()) {
            Module module = new Module(engine, WAT_BINARY);
            module.close();
        }
    }
    public static void testFunc() {
        byte[] WAT_BYTES_ADD = ("(module"
                + "  (func (export \"add\") (param $p1 i32) (param $p2 i32) (result i32)"
                + "    local.get $p1"
                + "    local.get $p2"
                + "    i32.add)"
                + ')').getBytes();
        try (Store store = new Store();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_ADD);
             Instance instance = new Instance(store, module, Collections.emptyList())) {
            try (Func func = instance.getFunc("add").get()) {
                WasmFunctions.Function2<Integer, Integer, Integer> add = WasmFunctions.func(func, I32, I32, I32);
                System.out.println(add.call(random.nextInt(),random.nextInt()).intValue());
            }
        }
    }
    public static void testWrapFunction() {
        byte[] WAT_BYTES_TRAMPOLINE = ("(module"
                + "  (func $callback (import \"\" \"callback\") (param i64 i64) (result i64))"
                + "  (func (export \"trampoline\") (param $p1 i64) (param $p2 i64) (result i64)"
                + "    local.get $p1"
                + "    local.get $p2"
                + "    call $callback)"
                + ')').getBytes();
        try (Store store = new Store();
             Engine engine = store.engine();
             Module module = new Module(engine, WAT_BYTES_TRAMPOLINE);
             Func callback = WasmFunctions.wrap(store, I64, I64, I64, (lhs, rhs) -> lhs + rhs);
             Instance instance = new Instance(store, module, Arrays.asList(Extern.fromFunc(callback)))) {
            try (Func func = instance.getFunc("trampoline").get()) {
                WasmFunctions.Function2<Long, Long, Long> trampoline = WasmFunctions.func(func, I64, I64, I64);
                long sum = trampoline.call(random.nextLong(),random.nextLong()).longValue();
                System.out.println(sum);
            }
        }
    }
}
