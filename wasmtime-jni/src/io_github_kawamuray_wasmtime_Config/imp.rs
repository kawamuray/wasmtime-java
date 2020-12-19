use super::JniEngine;
use crate::errors;
use crate::interop;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jlong, jobject, jboolean};
use jni::{self, JNIEnv};
use wasmtime::{Engine, Config};
use std::sync::MutexGuard;
use std::convert::TryInto;

pub(super) struct JniEngineImpl;

impl<'a> JniEngine<'a> for JniEngineImpl {
    type Error = errors::Error;
    fn cache_config_load(env: &JNIEnv, this: JObject, path: JObject) -> Result<jobject, Self::Error>{
        let mut config = interop::get_inner::<Config>(env,this)?;
        //TODO: How to transmute a JValue to AsRef<Path> ?
        config.cache_config_load(env.call_method(path,"toString","(V)Ljava/lang/String;",&[]));
        return Ok(this?.into_inner())
    }
    fn cache_config_load_default(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error>{
        let mut config = interop::get_inner::<Config>(env,this)?;
        config.cache_config_load_default();
        return Ok(this?.into_inner())
    }
    fn cranelift_debug_verifier(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>{
        let mut config = interop::get_inner::<Config>(env,this)?;
        config.cranelift_debug_verifier(enable==1);
        return Ok(this?.into_inner())
    }
    fn cranelift_nan_canonicalization(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>{
        let mut config = interop::get_inner::<Config>(env,this)?;
        config.cranelift_nan_canonicalization(enable==1);
        return Ok(this?.into_inner())
    }
    fn cranelift_opt_level(env: &JNIEnv, this: JObject, level: JObject) -> Result<jobject, Self::Error>{
        let mut config = interop::get_inner::<Config>(env,this)?;
        let mut ordinal = env.call_method(level,"ordinal","(I)V",&[]);
    }
    fn cranelift_other_flag(env: &JNIEnv, this: JObject, name: JString, value: JString) -> Result<jobject, Self::Error>{

    }
    fn debug_info(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>{

    }
    fn dynamic_memory_guard_size(env: &JNIEnv, this: JObject, guard_size: jlong) -> Result<jobject, Self::Error>{

    }
    fn interruptable(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>{

    }
    fn max_wasm_stack(env: &JNIEnv, this: JObject, size: jlong) -> Result<jobject, Self::Error>{

    }
    fn new_config(env: &JNIEnv, clazz: JClass) -> Result<jlong, Self::Error>{
        let config = Config::default();
        Ok(interop::into_raw::<Config>(config))
    }
    fn profiler(env: &JNIEnv, this: JObject, profile: JObject) -> Result<jobject, Self::Error>{

    }
    fn static_memory_guard_size(env: &JNIEnv, this: JObject, guard_size: jlong) -> Result<jobject, Self::Error>{

    }
    fn static_memory_maximum_size(env: &JNIEnv, this: JObject, max_size: jlong) -> Result<jobject, Self::Error>{

    }
    fn strategy(env: &JNIEnv, this: JObject, strategy: JObject) -> Result<jobject, Self::Error>{

    }
    fn wasm_bulk_memory(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>{

    }
    fn wasm_multi_value(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>{

    }
    fn wasm_reference_types(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>{

    }
    fn wasm_simd(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>{

    }
    fn wasm_threads(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>{

    }
}
