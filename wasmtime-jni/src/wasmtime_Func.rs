use crate::errors::{self, Result};
use crate::{interop, utils, wrap_error, wtrap, wval};
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong, jobjectArray};
use jni::{JNIEnv, JavaVM};
use wasmtime::{Caller, Func, FuncType, Store, Trap, Val};

struct FuncFinalizer {
    jvm: JavaVM,
    index: jint,
}

impl Drop for FuncFinalizer {
    fn drop(&mut self) {
        let env = self.jvm.attach_current_thread().unwrap();
        env.call_static_method(
            "wasmtime/Func",
            "dropTrampoline",
            "(I)V",
            &[self.index.into()],
        )
        .unwrap();
    }
}

fn invoke_trampoline<'a>(
    jvm: &JavaVM,
    index: jint,
    caller: Caller<'a>,
    params: &[Val],
    results: &mut [Val],
    // Just to capture it in closure calling this so that it drops alongs with closure
    _finalizer: &FuncFinalizer,
) -> Result<Option<Trap>> {
    // TODO: this should be attach_current_thread_permanently?
    let env = jvm.attach_current_thread()?;

    // Convert Rust param values into Java's
    let jparams = utils::into_java_array(
        &env,
        "wasmtime/Val",
        params
            .into_iter()
            .map(|param| wval::into_java(&env, param.clone()))
            .collect::<Result<Vec<_>, _>>()?,
    )?;
    let jresults = env.new_object_array(results.len() as jint, "wasmtime/Val", JObject::null())?;
    let caller_ptr = &caller as *const Caller as jlong;

    let trap = env
        .call_static_method(
            "wasmtime/Func",
            "invokeTrampoline",
            "(JI[Lwasmtime/Val;[Lwasmtime/Val;)Lwasmtime/Trap;",
            &[
                caller_ptr.into(),
                index.into(),
                jparams.into(),
                jresults.into(),
            ],
        )?
        .l()?;

    // Fill java results value into Rust's
    for (i, rval) in utils::JavaArrayIter::new(&env, jresults)?.enumerate() {
        let rval = rval?;
        if !rval.is_null() {
            results[i] = wval::from_java(&env, rval)?;
        }
    }
    Ok(if trap.is_null() {
        None
    } else {
        Some(wtrap::from_java(&env, trap)?)
    })
}

fn new_func(env: &JNIEnv, store_ptr: jlong, fn_type: JObject, index: jint) -> Result<jlong> {
    let store = interop::ref_from_raw::<Store>(store_ptr)?;

    let param_types = wval::types_from_java(
        &env,
        env.get_field(fn_type, "params", "[Lwasmtime/Val$Type;")?
            .l()?
            .into_inner(),
    )?
    .into_boxed_slice();
    let result_types = wval::types_from_java(
        &env,
        env.get_field(fn_type, "results", "[Lwasmtime/Val$Type;")?
            .l()?
            .into_inner(),
    )?
    .into_boxed_slice();
    let fn_type = FuncType::new(param_types, result_types);

    let jvm = env.get_java_vm()?;
    let finalizer = FuncFinalizer {
        jvm: env.get_java_vm()?,
        index,
    };
    let func = Func::new(&store, fn_type, move |caller, params, results| {
        let ret = invoke_trampoline(&jvm, index, caller, params, results, &finalizer);
        match ret {
            Ok(None) => Ok(()),
            Ok(Some(trap)) => Err(trap),
            Err(e) => Err(Trap::new(e.to_string())),
        }
    });

    Ok(interop::into_raw::<Func>(func))
}

#[no_mangle]
extern "system" fn Java_wasmtime_Func_newFunc(
    env: JNIEnv,
    _clazz: JClass,
    store_ptr: jlong,
    fn_type: JObject,
    index: jint,
) -> jlong {
    wrap_error!(env, new_func(&env, store_ptr, fn_type, index))
}

fn call_func(env: &JNIEnv, this: JObject, params: jobjectArray) -> Result<jobjectArray> {
    let func = interop::get_inner::<Func>(&env, this)?;

    let iter = utils::JavaArrayIter::new(env, params)?;
    let mut wasm_params = Vec::with_capacity(iter.len());
    for val in iter {
        let wasm_val = wval::from_java(env, val?)?;
        wasm_params.push(wasm_val);
    }

    // TODO: it might be better to convert the error into a dedicated exception for Trap
    let results = func.call(&wasm_params)?;

    let java_results = results
        .into_iter()
        .map(|wasm_val| wval::into_java(env, wasm_val.clone()))
        .collect::<Result<Vec<_>, _>>()?;

    utils::into_java_array(env, "wasmtime/Val", java_results)
}

#[no_mangle]
extern "system" fn Java_wasmtime_Func_nativeCall(
    env: JNIEnv,
    this: JObject,
    params: jobjectArray,
) -> jobjectArray {
    match call_func(&env, this, params) {
        Ok(array) => array,
        Err(e) => {
            errors::error_to_exception(&env, e);
            JObject::null().into_inner()
        }
    }
}

#[no_mangle]
extern "system" fn Java_wasmtime_Func_dispose(env: JNIEnv, this: JObject) {
    wrap_error!(env, interop::take_inner::<Func>(&env, this));
}
