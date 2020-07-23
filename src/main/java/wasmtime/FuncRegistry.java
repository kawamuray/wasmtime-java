package wasmtime;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicInteger;

import wasmtime.Func.Handler;

class FuncRegistry {
    private final ConcurrentMap<Integer, Handler> map;
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
