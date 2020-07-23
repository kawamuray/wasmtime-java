package wasmtime;

public interface WasmValType<T> {
    class I32 implements WasmValType<Integer> {
        @Override
        public Val.Type type() {
            return Val.Type.I32;
        }
        @Override
        public Integer fromWasmVal(Val val) {
            return val.i32();
        }
        @Override
        public Val toWasmVal(Integer val) {
            return Val.fromI32(val);
        }
    }

    class I64 implements WasmValType<Long> {
        @Override
        public Val.Type type() {
            return Val.Type.I64;
        }
        @Override
        public Long fromWasmVal(Val val) {
            return val.i64();
        }
        @Override
        public Val toWasmVal(Long val) {
            return Val.fromI64(val);
        }
    }

    class F32 implements WasmValType<Float> {
        @Override
        public Val.Type type() {
            return Val.Type.F32;
        }
        @Override
        public Float fromWasmVal(Val val) {
            return val.f32();
        }
        @Override
        public Val toWasmVal(Float val) {
            return Val.fromF32(val);
        }
    }

    class F64 implements WasmValType<Double> {
        @Override
        public Val.Type type() {
            return Val.Type.F64;
        }
        @Override
        public Double fromWasmVal(Val val) {
            return val.f64();
        }
        @Override
        public Val toWasmVal(Double val) {
            return Val.fromF64(val);
        }
    }

    WasmValType<Integer> I32 = new I32();
    WasmValType<Long> I64 = new I64();
    WasmValType<Float> F32 = new F32();
    WasmValType<Double> F64 = new F64();

    Val.Type type();

    T fromWasmVal(Val val);

    Val toWasmVal(T val);
}
