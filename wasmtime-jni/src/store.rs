use wasi_common::WasiCtx;

pub(crate) struct StoreData {
    pub wasi: Option<WasiCtx>,
}
