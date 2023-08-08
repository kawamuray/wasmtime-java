// THIS FILE IS GENERATED AUTOMATICALLY. DO NOT EDIT!
mod imp;

use self::imp::JniLinkerImpl;
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

trait JniLinker<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error>;
    fn native_define(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        module_name: JString<'a>,
        name: JString<'a>,
        extern_item: JObject<'a>,
    ) -> Result<(), Self::Error>;
    fn native_externs(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
    ) -> Result<jobjectArray, Self::Error>;
    fn native_get(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        module: JString<'a>,
        name: JString<'a>,
    ) -> Result<jobject, Self::Error>;
    fn native_module(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        module_name: JString<'a>,
        module_ptr: jlong,
    ) -> Result<(), Self::Error>;
    fn new_linker(
        env: &mut JNIEnv<'a>,
        clazz: JClass<'a>,
        engine_ptr: jlong,
    ) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Linker_dispose<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
) {
    wrap_error!(
        env,
        JniLinkerImpl::dispose(&mut env, this),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Linker_nativeDefine__JLjava_lang_String_2Ljava_lang_String_2Lio_github_kawamuray_wasmtime_Extern_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
    module_name: JString<'a>,
    name: JString<'a>,
    extern_item: JObject<'a>,
) {
    wrap_error!(
        env,
        JniLinkerImpl::native_define(&mut env, this, store_ptr, module_name, name, extern_item),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Linker_nativeExterns__J<'a>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
) -> jobjectArray {
    wrap_error!(
        env,
        JniLinkerImpl::native_externs(&mut env, this, store_ptr),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Linker_nativeGet__JLjava_lang_String_2Ljava_lang_String_2<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
    module: JString<'a>,
    name: JString<'a>,
) -> jobject {
    wrap_error!(
        env,
        JniLinkerImpl::native_get(&mut env, this, store_ptr, module, name),
        JObject::null().into_raw()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Linker_nativeModule__JLjava_lang_String_2J<
    'a,
>(
    mut env: JNIEnv<'a>,
    this: JObject<'a>,
    store_ptr: jlong,
    module_name: JString<'a>,
    module_ptr: jlong,
) {
    wrap_error!(
        env,
        JniLinkerImpl::native_module(&mut env, this, store_ptr, module_name, module_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_io_github_kawamuray_wasmtime_Linker_newLinker__J<'a>(
    mut env: JNIEnv<'a>,
    clazz: JClass<'a>,
    engine_ptr: jlong,
) -> jlong {
    wrap_error!(
        env,
        JniLinkerImpl::new_linker(&mut env, clazz, engine_ptr),
        Default::default()
    )
}
