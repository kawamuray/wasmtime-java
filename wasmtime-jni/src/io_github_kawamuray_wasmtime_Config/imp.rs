use crate::errors;
use crate::interop;
use crate::io_github_kawamuray_wasmtime_Config::JniConfig;
use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jboolean, jlong, jobject};
use jni::{self, JNIEnv};
use std::path::Path;
use std::result::Result;
use wasmtime::{Config, OptLevel, ProfilingStrategy, Strategy};

pub(super) struct JniConfigImpl;

impl<'a> JniConfig<'a> for JniConfigImpl {
    type Error = errors::Error;
    fn cache_config_load(
        env: &JNIEnv,
        this: JObject,
        path: JObject,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        let re = env.call_method(path, "toString", "(V)Ljava/lang/String;", &[]);
        let j_obj = re.unwrap().l().unwrap();
        let j_str = JString::from(j_obj);
        let path_java_str = env.get_string(j_str).unwrap();
        let path_str = path_java_str.to_str().unwrap();
        config.cache_config_load(Path::new(path_str))?;
        return Ok(this.into_inner());
    }
    fn cache_config_load_default(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.cache_config_load_default()?;
        return Ok(this.into_inner());
    }
    fn cranelift_debug_verifier(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.cranelift_debug_verifier(enable == 1);
        return Ok(this.into_inner());
    }
    fn cranelift_nan_canonicalization(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.cranelift_nan_canonicalization(enable == 1);
        return Ok(this.into_inner());
    }
    fn cranelift_opt_level(
        env: &JNIEnv,
        this: JObject,
        level: JObject,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        let ordinal = env.call_method(level, "ordinal", "()I", &[])?.i()?;
        let optlevel: OptLevel;
        match ordinal {
            0 => optlevel = OptLevel::None,
            1 => optlevel = OptLevel::Speed,
            2 => optlevel = OptLevel::SpeedAndSize,
            _ => optlevel = OptLevel::Speed,
        }
        config.cranelift_opt_level(optlevel);
        return Ok(this.into_inner());
    }
    fn cranelift_other_flag(
        env: &JNIEnv,
        this: JObject,
        name: JString,
        value: JString,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        let name_j_str = env.get_string(name).unwrap();
        let name_str = name_j_str.to_str().unwrap();
        let value_j_str = env.get_string(value).unwrap();
        let value_str = value_j_str.to_str().unwrap();
        unsafe {
            config.cranelift_other_flag(name_str, value_str)?;
        }
        return Ok(this.into_inner());
    }
    fn debug_info(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.debug_info(enable == 1);
        return Ok(this.into_inner());
    }
    fn dynamic_memory_guard_size(
        env: &JNIEnv,
        this: JObject,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.dynamic_memory_guard_size(guard_size as u64);
        return Ok(this.into_inner());
    }
    fn interruptable(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.interruptable(enable == 1);
        return Ok(this.into_inner());
    }
    fn max_wasm_stack(env: &JNIEnv, this: JObject, size: jlong) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.max_wasm_stack(size as usize);
        return Ok(this.into_inner());
    }
    fn new_config(_env: &JNIEnv, _clazz: JClass) -> Result<jlong, Self::Error> {
        let config = Config::default();
        return Ok(interop::into_raw::<Config>(config));
    }
    fn profiler(env: &JNIEnv, this: JObject, profile: JObject) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        let ordinal = env.call_method(profile, "ordinal", "()I", &[])?.i()?;
        let profiling_strategy: ProfilingStrategy;
        match ordinal {
            0 => profiling_strategy = ProfilingStrategy::None,
            1 => profiling_strategy = ProfilingStrategy::JitDump,
            2 => profiling_strategy = ProfilingStrategy::VTune,
            _ => profiling_strategy = ProfilingStrategy::None,
        }
        config.profiler(profiling_strategy)?;
        return Ok(this.into_inner());
    }
    fn static_memory_guard_size(
        env: &JNIEnv,
        this: JObject,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.static_memory_guard_size(guard_size as u64);
        return Ok(this.into_inner());
    }
    fn static_memory_maximum_size(
        env: &JNIEnv,
        this: JObject,
        max_size: jlong,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.static_memory_maximum_size(max_size as u64);
        return Ok(this.into_inner());
    }
    fn strategy(env: &JNIEnv, this: JObject, strategy: JObject) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        let ordinal = env.call_method(strategy, "ordinal", "()I", &[])?.i()?;
        let strategy: Strategy;
        match ordinal {
            0 => strategy = Strategy::Auto,
            1 => strategy = Strategy::Cranelift,
            2 => strategy = Strategy::Lightbeam,
            _ => strategy = Strategy::Auto,
        }
        config.strategy(strategy)?;
        return Ok(this.into_inner());
    }
    fn wasm_bulk_memory(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_bulk_memory(enable == 1);
        return Ok(this.into_inner());
    }
    fn wasm_multi_value(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_multi_value(enable == 1);
        return Ok(this.into_inner());
    }
    fn wasm_reference_types(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_reference_types(enable == 1);
        return Ok(this.into_inner());
    }
    fn wasm_simd(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_simd(enable == 1);
        return Ok(this.into_inner());
    }
    fn wasm_threads(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error> {
        let mut config = interop::get_inner::<Config>(env, this)?;
        config.wasm_threads(enable == 1);
        return Ok(this.into_inner());
    }
}
