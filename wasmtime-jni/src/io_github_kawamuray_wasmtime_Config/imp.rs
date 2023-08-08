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
    fn cache_config_load_default(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.cache_config_load_default()?;
        Ok(this.into_raw())
    }
    fn cache_config_load_native(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        path: JString<'a>,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        let path_j_str = env.get_string(&path)?;
        //the trait `From<Utf8Error>` is not implemented for `errors::Error`
        //So the `?` operator cannot be used at here
        let path_str = path_j_str.to_str().expect("error path!");
        config.cache_config_load(Path::new(path_str))?;
        Ok(this.into_raw())
    }
    fn cranelift_debug_verifier(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.cranelift_debug_verifier(enable == 1);
        Ok(this.into_raw())
    }
    fn cranelift_nan_canonicalization(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.cranelift_nan_canonicalization(enable == 1);
        Ok(this.into_raw())
    }
    fn cranelift_opt_level(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        level: JObject<'a>,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        let enum_string = utils::enum_name(env, level)?;
        let optlevel: OptLevel = match enum_string.as_str() {
            "NONE" => OptLevel::None,
            "SPEED" => OptLevel::Speed,
            "SPEED_AND_SIZE" => OptLevel::SpeedAndSize,
            _ => OptLevel::Speed,
        };
        config.cranelift_opt_level(optlevel);
        Ok(this.into_raw())
    }
    fn debug_info(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.debug_info(enable == 1);
        Ok(this.into_raw())
    }
    fn dynamic_memory_guard_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.dynamic_memory_guard_size(guard_size as u64);
        Ok(this.into_raw())
    }
    fn epoch_interruption(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.epoch_interruption(enable == 1);
        Ok(this.into_raw())
    }
    fn max_wasm_stack(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.max_wasm_stack(size as usize);
        Ok(this.into_raw())
    }
    fn new_config(_env: &mut JNIEnv<'a>, _clazz: JClass) -> Result<jlong, Self::Error> {
        let config = Config::default();
        Ok(interop::into_raw::<Config>(config))
    }
    fn profiler(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        profile: JObject,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        let enum_string = utils::enum_name(env, profile)?;
        let profiling_strategy = match enum_string.as_str() {
            "NONE" => ProfilingStrategy::None,
            "JIT_DUMP" => ProfilingStrategy::JitDump,
            "V_TUNE" => ProfilingStrategy::VTune,
            _ => ProfilingStrategy::None,
        };
        config.profiler(profiling_strategy);
        Ok(this.into_raw())
    }
    fn static_memory_guard_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.static_memory_guard_size(guard_size as u64);
        Ok(this.into_raw())
    }
    fn static_memory_maximum_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        max_size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.static_memory_maximum_size(max_size as u64);
        Ok(this.into_raw())
    }
    fn strategy(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        strategy: JObject<'a>,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        let enum_string = utils::enum_name(env, strategy)?;
        let strategy: Strategy = match enum_string.as_str() {
            "AUTO" => Strategy::Auto,
            "CRANELIFT" => Strategy::Cranelift,
            _ => Strategy::Auto,
        };
        config.strategy(strategy);
        Ok(this.into_raw())
    }
    fn wasm_bulk_memory(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.wasm_bulk_memory(enable == 1);
        Ok(this.into_raw())
    }
    fn wasm_multi_value(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.wasm_multi_value(enable == 1);
        Ok(this.into_raw())
    }
    fn wasm_reference_types(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.wasm_reference_types(enable == 1);
        Ok(this.into_raw())
    }
    fn wasm_simd(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.wasm_simd(enable == 1);
        Ok(this.into_raw())
    }
    fn wasm_threads(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, &this)?;
        config.wasm_threads(enable == 1);
        Ok(this.into_raw())
    }
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error> {
        interop::dispose_inner::<Config>(env, &this)?;
        Ok(())
    }
}
