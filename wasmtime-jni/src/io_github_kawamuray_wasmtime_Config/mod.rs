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
                if let Err(err) = $env.throw(e) {
                    $env.exception_describe().ok();
                    panic!("error in throwing exception: {}", err);
                }
                $default
            }
        }
    };
}

trait JniConfig<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn cache_config_load_default(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
    ) -> Result<jobject, Self::Error>;
    fn cache_config_load_native(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        path: JString<'a>,
    ) -> Result<jobject, Self::Error>;
    fn cranelift_debug_verifier(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn cranelift_nan_canonicalization(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn cranelift_opt_level(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        level: JObject<'a>,
    ) -> Result<jobject, Self::Error>;
    fn debug_info(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn dynamic_memory_guard_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error>;
    fn epoch_interruption(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn max_wasm_stack(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        size: jlong,
    ) -> Result<jobject, Self::Error>;
    fn new_config(env: &mut JNIEnv<'a>, clazz: JClass<'a>) -> Result<jlong, Self::Error>;
    fn profiler(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        profile: JObject<'a>,
    ) -> Result<jobject, Self::Error>;
    fn static_memory_guard_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        guard_size: jlong,
    ) -> Result<jobject, Self::Error>;
    fn static_memory_maximum_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        max_size: jlong,
    ) -> Result<jobject, Self::Error>;
    fn strategy(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        strategy: JObject<'a>,
    ) -> Result<jobject, Self::Error>;
    fn wasm_bulk_memory(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn wasm_multi_value(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn wasm_reference_types(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn wasm_simd(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
    fn wasm_threads(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        enable: jboolean,
    ) -> Result<jobject, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_cacheConfigLoadDefault<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cache_config_load_default(&mut env, this),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_cacheConfigLoadNative__Ljava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    path: JString<'a>,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cache_config_load_native(&mut env, this, path),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_craneliftDebugVerifier__Z<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cranelift_debug_verifier(&mut env, this, enable),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_craneliftNanCanonicalization__Z<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cranelift_nan_canonicalization(&mut env, this, enable),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_craneliftOptLevel__Lio_github_kawamuray_wasmtime_OptLevel_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    level: JObject<'a>,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::cranelift_opt_level(&mut env, this, level),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_debugInfo__Z<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::debug_info(&mut env, this, enable),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniConfigImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_dynamicMemoryGuardSize__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    guard_size: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::dynamic_memory_guard_size(&mut env, this, guard_size),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_epochInterruption__Z<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::epoch_interruption(&mut env, this, enable),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_maxWasmStack__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    size: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::max_wasm_stack(&mut env, this, size),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_newConfig<'a>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
) -> jlong {
    wrap_error!(
        env,
        JniConfigImpl::new_config(&mut env, clazz),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_profiler__Lio_github_kawamuray_wasmtime_ProfilingStrategy_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    profile: JObject<'a>,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::profiler(&mut env, this, profile),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_staticMemoryGuardSize__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    guard_size: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::static_memory_guard_size(&mut env, this, guard_size),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_staticMemoryMaximumSize__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    max_size: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::static_memory_maximum_size(&mut env, this, max_size),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_strategy__Lio_github_kawamuray_wasmtime_Strategy_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    strategy: JObject<'a>,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::strategy(&mut env, this, strategy),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmBulkMemory__Z<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_bulk_memory(&mut env, this, enable),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmMultiValue__Z<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_multi_value(&mut env, this, enable),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmReferenceTypes__Z<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_reference_types(&mut env, this, enable),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmSimd__Z<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_simd(&mut env, this, enable),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Config_wasmThreads__Z<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    enable: jboolean,
) -> jobject {
    wrap_error!(
        env,
        JniConfigImpl::wasm_threads(&mut env, this, enable),
        JObject::null().into_raw()
    )
}
