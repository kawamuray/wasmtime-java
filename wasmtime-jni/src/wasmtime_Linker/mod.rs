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
                $env.throw(e).expect("error in throwing exception");
                $default
            }
        }
    };
}

trait JniLinker<'a> {
    type Error: Desc<'a, JThrowable<'a>>;
    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error>;
    fn native_define(
        env: &JNIEnv,
        this: JObject,
        module_name: JString,
        name: JString,
        extern_item: JObject,
    ) -> Result<(), Self::Error>;
    fn native_get_one_by_name(
        env: &JNIEnv,
        this: JObject,
        module: JString,
        name: JString,
    ) -> Result<jobject, Self::Error>;
    fn native_module(
        env: &JNIEnv,
        this: JObject,
        module_name: JString,
        module_ptr: jlong,
    ) -> Result<(), Self::Error>;
    fn new_linker(env: &JNIEnv, clazz: JClass, store_ptr: jlong) -> Result<jlong, Self::Error>;
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, JniLinkerImpl::dispose(&env, this), Default::default())
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_nativeDefine(
    env: JNIEnv,
    this: JObject,
    module_name: JString,
    name: JString,
    extern_item: JObject,
) {
    wrap_error!(
        env,
        JniLinkerImpl::native_define(&env, this, module_name, name, extern_item),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_nativeGetOneByName(
    env: JNIEnv,
    this: JObject,
    module: JString,
    name: JString,
) -> jobject {
    wrap_error!(
        env,
        JniLinkerImpl::native_get_one_by_name(&env, this, module, name),
        JObject::null().into_inner()
    )
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_nativeModule(
    env: JNIEnv,
    this: JObject,
    module_name: JString,
    module_ptr: jlong,
) {
    wrap_error!(
        env,
        JniLinkerImpl::native_module(&env, this, module_name, module_ptr),
        Default::default()
    )
}

#[no_mangle]
extern "system" fn Java_wasmtime_Linker_newLinker(
    env: JNIEnv,
    clazz: JClass,
    store_ptr: jlong,
) -> jlong {
    wrap_error!(
        env,
        JniLinkerImpl::new_linker(&env, clazz, store_ptr),
        Default::default()
    )
}
