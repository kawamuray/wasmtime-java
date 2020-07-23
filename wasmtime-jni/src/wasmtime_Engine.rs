use crate::interop;
use crate::wrap_error;
use jni::objects::{JClass, JObject};
use jni::sys::jlong;
use jni::{self, JNIEnv};
use wasmtime::Engine;

#[no_mangle]
extern "system" fn Java_wasmtime_Engine_newEngine(_env: JNIEnv, _clazz: JClass) -> jlong {
    let engine = Engine::default();
    interop::into_raw(engine)
}

#[no_mangle]
extern "system" fn Java_wasmtime_Engine_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, interop::take_inner::<Engine>(&env, this));
}
