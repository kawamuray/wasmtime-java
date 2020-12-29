// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniConfigImpl;
use jni::descriptors::Desc;
use jni::objects::*;
use jni::sys::*;
use jni::JNIEnv;

macro_rules! wrap_error {
    ($env:expr, $body:expr, $default:expr) => {
        match $body {
            Ok(v) => v,
            Err(e) => {
                $env.throw(e).expect("error in throwing exception");
                $default
            }
        }
    };
}

trait JniConfig<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn cache_config_load_default(env: &JNIEnv, this: JObject) -> Result<jobject, Self::Error>;
    fn cache_config_load_native(
        env: &JNIEnv,
        this: JObject,
        path: JString,
    ) -> Result<jobject, Self::Error>;
    fn cranelift_debug_verifier(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn cranelift_nan_canonicalization(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn cranelift_opt_level(
        env: &JNIEnv,
        this: JObject,
        level: JObject,
    ) -> Result<jobject, Self::Error>;
    fn cranelift_other_flag(
        env: &JNIEnv,
        this: JObject,
        name: JString,
        value: JString,
    ) -> Result<jobject, Self::Error>;
    fn debug_info(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn dynamic_memory_guard_size(
        env: &JNIEnv,
        this: JObject,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error>;
    fn interruptable(env: &JNIEnv, this: JObject, enable: jboolean)
        -> Result<jobject, Self::Error>;
    fn max_wasm_stack(env: &JNIEnv, this: JObject, size: jlong) -> Result<jobject, Self::Error>;
    fn new_config(env: &JNIEnv, clazz: JClass) -> Result<jlong, Self::Error>;
    fn profiler(env: &JNIEnv, this: JObject, profile: JObject) -> Result<jobject, Self::Error>;
    fn static_memory_guard_size(
        env: &JNIEnv,
        this: JObject,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error>;
    fn static_memory_maximum_size(
        env: &JNIEnv,
        this: JObject,
        max_size: jlong,
    ) -> Result<jobject, Self::Error>;
    fn strategy(env: &JNIEnv, this: JObject, strategy: JObject) -> Result<jobject, Self::Error>;
    fn wasm_bulk_memory(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn wasm_multi_value(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn wasm_reference_types(
        env: &JNIEnv,
        this: JObject,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn wasm_simd(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>;
    fn wasm_threads(env: &JNIEnv, this: JObject, enable: jboolean) -> Result<jobject, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_cacheConfigLoadDefault(
    env: JNIEnv,
    this: JObject,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cache_config_load_default(&env, this),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_cacheConfigLoadNative(
    env: JNIEnv,
    this: JObject,
    path: JString,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cache_config_load_native(&env, this, path),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_craneliftDebugVerifier(
    env: JNIEnv,
    this: JObject,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cranelift_debug_verifier(&env, this, enable),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_craneliftNanCanonicalization(
    env: JNIEnv,
    this: JObject,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cranelift_nan_canonicalization(&env, this, enable),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_craneliftOptLevel(
    env: JNIEnv,
    this: JObject,
    level: JObject,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cranelift_opt_level(&env, this, level),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_craneliftOtherFlag(
    env: JNIEnv,
    this: JObject,
    name: JString,
    value: JString,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cranelift_other_flag(&env, this, name, value),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_debugInfo(
    env: JNIEnv,
    this: JObject,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::debug_info(&env, this, enable),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniConfigImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_dynamicMemoryGuardSize(
    env: JNIEnv,
    this: JObject,
    guard_size: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::dynamic_memory_guard_size(&env, this, guard_size),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_interruptable(
    env: JNIEnv,
    this: JObject,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::interruptable(&env, this, enable),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_maxWasmStack(
    env: JNIEnv,
    this: JObject,
    size: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::max_wasm_stack(&env, this, size),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_newConfig(
    env: JNIEnv,
    clazz: JClass,
) -> jlong {
    wrap_error!(
        env,
        JniConfigImpl::new_config(&env, clazz),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_profiler(
    env: JNIEnv,
    this: JObject,
    profile: JObject,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::profiler(&env, this, profile),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_staticMemoryGuardSize(
    env: JNIEnv,
    this: JObject,
    guard_size: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::static_memory_guard_size(&env, this, guard_size),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_staticMemoryMaximumSize(
    env: JNIEnv,
    this: JObject,
    max_size: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::static_memory_maximum_size(&env, this, max_size),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_strategy(
    env: JNIEnv,
    this: JObject,
    strategy: JObject,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::strategy(&env, this, strategy),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmBulkMemory(
    env: JNIEnv,
    this: JObject,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_bulk_memory(&env, this, enable),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmMultiValue(
    env: JNIEnv,
    this: JObject,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_multi_value(&env, this, enable),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmReferenceTypes(
    env: JNIEnv,
    this: JObject,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_reference_types(&env, this, enable),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmSimd(
    env: JNIEnv,
    this: JObject,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_simd(&env, this, enable),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmThreads(
    env: JNIEnv,
    this: JObject,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_threads(&env, this, enable),
        JObject::null().into_inner()
    )
}
