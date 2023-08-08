// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniMemoryImpl;
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

trait JniMemory<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn native_buffer(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jobject, Self::Error>;
    fn native_data_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jlong, Self::Error>;
    fn native_grow(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        delta_pages: jlong,
    ) -> Result<jint, Self::Error>;
    fn native_size(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jint, Self::Error>;
    fn new_memory(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        inner_ptr: jlong,
        minimum: jlong,
        maximum: jlong,
        is64: jboolean,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniMemoryImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_nativeBuffer__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
) -> jobject {
    wrap_error!(
        env,
        JniMemoryImpl::native_buffer(&mut env, this, store_ptr),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_nativeDataSize__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
) -> jlong {
    wrap_error!(
        env,
        JniMemoryImpl::native_data_size(&mut env, this, store_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_nativeGrow__JJ<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
    delta_pages: jlong,
) -> jint {
    wrap_error!(
        env,
        JniMemoryImpl::native_grow(&mut env, this, store_ptr, delta_pages),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_nativeSize__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
) -> jint {
    wrap_error!(
        env,
        JniMemoryImpl::native_size(&mut env, this, store_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Memory_newMemory__JJJZ<'a>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    inner_ptr: jlong,
    minimum: jlong,
    maximum: jlong,
    is64: jboolean,
) -> jlong {
    wrap_error!(
        env,
        JniMemoryImpl::new_memory(&mut env, clazz, inner_ptr, minimum, maximum, is64),
        Default::default()
    )
}
