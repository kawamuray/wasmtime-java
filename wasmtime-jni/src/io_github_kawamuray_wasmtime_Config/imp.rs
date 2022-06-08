use crate::errors;
use crate::interop;
use crate::io_github_kawamuray_wasmtime_Config::JniConfig;
use crate::utils;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jlong, jobject};
use jni::{self, JNIEnv};
use std::path::Path;
use std::result::Result;
use wasmtime::{Config, OptLevel, ProfilingStrategy, Strategy};

pub(super) struct JniConfigImpl;

impl<'a> JniConfig<'a> for JniConfigImpl {
    type Error = errors::Error;
    fn cache_config_load_default(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.cache_config_load_default()?;
        Ok(this.into_inner())
    }
    fn cache_config_load_native(
        env: &JNIEnv,
        this: JObject,
        path: JString,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        let path_j_str = env.get_string(path)?;
        //the trait `From<Utf8Error>` is not implemented for `errors::Error`
        //So the `?` operator cannot be used at here
        let path_str = path_j_str.to_str().expect("error path!");
        config.cache_config_load(Path::new(path_str))?;
        Ok(this.into_inner())
    }
    fn cranelift_debug_verifier(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.cranelift_debug_verifier(enable == 1);
        Ok(this.into_inner())
    }
    fn cranelift_nan_canonicalization(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.cranelift_nan_canonicalization(enable == 1);
        Ok(this.into_inner())
    }
    fn cranelift_opt_level(
        env: &JNIEnv,
        this: JObject,
        level: JObject,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        let enum_string = utils::enum_name(env, level)?;
        let optlevel: OptLevel = match enum_string.as_str() {
            "NONE" => OptLevel::None,
            "SPEED" => OptLevel::Speed,
            "SPEED_AND_SIZE" => OptLevel::SpeedAndSize,
            _ => OptLevel::Speed,
        };
        config.cranelift_opt_level(optlevel);
        Ok(this.into_inner())
    }
    fn debug_info(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.debug_info(enable == 1);
        Ok(this.into_inner())
    }
    fn dynamic_memory_guard_size(
        env: &JNIEnv,
        this: JObject,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.dynamic_memory_guard_size(guard_size as u64);
        Ok(this.into_inner())
    }
    fn interruptable(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.interruptable(enable == 1);
        Ok(this.into_inner())
    }
    fn max_wasm_stack(env: &JNIEnv, this: JObject, size: jlong) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.max_wasm_stack(size as usize)?;
        Ok(this.into_inner())
    }
    fn new_config(_env: &JNIEnv, _clazz: JClass) -> Result<jlong, Self::Error> {
        let config = Config::default();
        Ok(interop::into_raw::<Config>(config))
    }
    fn profiler(env: &JNIEnv, this: JObject, profile: JObject) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        let enum_string = utils::enum_name(env, profile)?;
        let profiling_strategy = match enum_string.as_str() {
            "NONE" => ProfilingStrategy::None,
            "JIT_DUMP" => ProfilingStrategy::JitDump,
            "V_TUNE" => ProfilingStrategy::VTune,
            _ => ProfilingStrategy::None,
        };
        config.profiler(profiling_strategy)?;
        Ok(this.into_inner())
    }
    fn static_memory_guard_size(
        env: &JNIEnv,
        this: JObject,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.static_memory_guard_size(guard_size as u64);
        Ok(this.into_inner())
    }
    fn static_memory_maximum_size(
        env: &JNIEnv,
        this: JObject,
        max_size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.static_memory_maximum_size(max_size as u64);
        Ok(this.into_inner())
    }
    fn strategy(env: &JNIEnv, this: JObject, strategy: JObject) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        let enum_string = utils::enum_name(env, strategy)?;
        let strategy: Strategy = match enum_string.as_str() {
            "AUTO" => Strategy::Auto,
            "CRANELIFT" => Strategy::Cranelift,
            _ => Strategy::Auto,
        };
        config.strategy(strategy)?;
        Ok(this.into_inner())
    }
    fn wasm_bulk_memory(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_bulk_memory(enable == 1);
        Ok(this.into_inner())
    }
    fn wasm_multi_value(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_multi_value(enable == 1);
        Ok(this.into_inner())
    }
    fn wasm_reference_types(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_reference_types(enable == 1);
        Ok(this.into_inner())
    }
    fn wasm_simd(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_simd(enable == 1);
        Ok(this.into_inner())
    }
    fn wasm_threads(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_threads(enable == 1);
        Ok(this.into_inner())
    }
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::dispose_inner::<Config>(&env, this)?;
        Ok(())
    }
}
