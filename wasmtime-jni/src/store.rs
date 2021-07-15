use jni::objects::GlobalRef;
use wasi_common::WasiCtx;

pub(crate) struct StoreData {
    pub wasi: Option<WasiCtx>,
    pub java_data: Option<GlobalRef>,
}
