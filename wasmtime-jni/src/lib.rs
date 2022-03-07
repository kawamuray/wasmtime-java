#![feature(thread_id_value)]
mod errors;
#[macro_use]
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Config;
pub(crate) mod interop;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Caller;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Engine;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Func;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Global;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Instance;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_InterruptHandle;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Linker;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Memory;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Module;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_Store;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_wasi_WasiCtx;
#[allow(non_snake_case)]
mod io_github_kawamuray_wasmtime_wasi_WasiCtxBuilder;
mod store;
pub(crate) mod utils;
mod wasi_utils;
mod wextern;
mod wtrap;
mod wval;
