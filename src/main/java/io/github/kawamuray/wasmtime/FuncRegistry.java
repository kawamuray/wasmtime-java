package io.github.kawamuray.wasmtime;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicInteger;

import io.github.kawamuray.wasmtime.Func.Handler;
import lombok.ToString;

@ToString
class FuncRegistry {
    // visible for testing
    final ConcurrentMap<Integer, Handler> map;
    private final AtomicInteger indexCounter;

    FuncRegistry() {
        map = new ConcurrentHashMap<>();
        indexCounter = new AtomicInteger();
    }

    int acquire(Handler handler) {
        int index = indexCounter.getAndIncrement();
        map.put(index, handler);
        return index;
    }

    Handler lookup(int index) {
        return map.get(index);
    }

    void drop(int index) {
        map.remove(index);
    }
}
