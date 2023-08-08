use super::JniFunc;
use crate::errors::{self, Result};
use crate::interop::ReentrantLock;
use crate::store::StoreData;
use crate::{interop, utils, wfuncerror, wval};
use anyhow::bail;
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong, jobjectArray};
use jni::{JNIEnv, JavaVM};
use wasmtime::{Caller, Func, FuncType, Store, Trap, Val};

pub(super) struct JniFuncImpl;

impl<'a> JniFunc<'a> for JniFuncImpl {
    type Error = errors::Error;

    fn new_func(
        env: &mut JNIEnv<'a>,
        _clazz: JClass<'a>,
        store_ptr: jlong,
        fn_type: JObject<'a>,
        index: jint,
    ) -> Result<jlong, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;

        let param_types_objs = env
            .get_field(
                &fn_type,
                "params",
                "[Lio/github/kawamuray/wasmtime/Val$Type;",
            )?
            .l()?
            .into_raw();
        let param_types = wval::types_from_java(env, param_types_objs)?.into_boxed_slice();
        let result_type_objs = env
            .get_field(
                &fn_type,
                "results",
                "[Lio/github/kawamuray/wasmtime/Val$Type;",
            )?
            .l()?
            .into_raw();
        let result_types = wval::types_from_java(env, result_type_objs)?.into_boxed_slice();
        let fn_type = FuncType::new(param_types.to_vec(), result_types.to_vec());

        let jvm = env.get_java_vm()?;
        let finalizer = FuncFinalizer {
            jvm: env.get_java_vm()?,
            index,
        };
        let func = Func::new(&mut *store, fn_type, move |caller, params, results| {
            let ret = invoke_trampoline(&jvm, index, caller, params, results, &finalizer);
            match ret {
                Ok(Ok(_)) => Ok(()),
                Ok(Err(wasm_error)) => Err(wasm_error),
                Err(e) => bail!(e.to_string()),
            }
        });

        Ok(interop::into_raw::<Func>(func))
    }

    fn native_call(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
        store_ptr: jlong,
        args: jobjectArray,
    ) -> Result<jobjectArray, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let func = interop::get_inner::<Func>(env, &this)?;

        let mut iter = utils::JavaArrayIter::new(env, args)?;
        let mut wasm_params = Vec::with_capacity(iter.len());
        while let Some(val) = iter.next(env) {
            let wasm_val = wval::from_java(env, val?)?;
            wasm_params.push(wasm_val);
        }
        let mut wasm_results =
            vec![unsafe { std::mem::zeroed() }; func.ty(&mut *store).results().len()];

        if let Err(e) = func.call(&mut *store, &wasm_params, &mut wasm_results) {
            return Err(if let Some(trap) = e.downcast_ref::<Trap>() {
                errors::Error::WasmTrap(*trap)
            } else if let Some(exit) = e.downcast_ref::<wasi_common::I32Exit>() {
                errors::Error::WasiI32ExitCode(exit.0)
            } else {
                e.into()
            });
        }

        let java_results = wasm_results
            .into_iter()
            .map(|wasm_val| wval::into_java(env, wasm_val.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        utils::into_java_array(env, "io/github/kawamuray/wasmtime/Val", java_results)
    }

    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error> {
        interop::dispose_inner::<Func>(env, &this)?;
        Ok(())
    }
}

struct FuncFinalizer {
    jvm: JavaVM,
    index: jint,
}

impl Drop for FuncFinalizer {
    fn drop(&mut self) {
        let mut env = self.jvm.attach_current_thread().unwrap();
        env.call_static_method(
            "io/github/kawamuray/wasmtime/Func",
            "dropTrampoline",
            "(I)V",
            &[self.index.into()],
        )
        .unwrap();
    }
}

fn invoke_trampoline<'a, T>(
    jvm: &JavaVM,
    index: jint,
    caller: Caller<'a, T>,
    params: &[Val],
    results: &mut [Val],
    // Just to capture it in closure calling this so that it drops alongs with closure
    _finalizer: &FuncFinalizer,
) -> Result<anyhow::Result<()>> {
    // TODO: this should be attach_current_thread_permanently?
    let mut env = jvm.attach_current_thread()?;

    // Convert Rust param values into Java's
    let jparam_values = params
        .into_iter()
        .map(|param| wval::into_java(&mut env, param.clone()))
        .collect::<Result<Vec<_>, _>>()?;
    let jparams =
        utils::into_java_array(&mut env, "io/github/kawamuray/wasmtime/Val", jparam_values)?;
    let jresults = env.new_object_array(
        results.len() as jint,
        "io/github/kawamuray/wasmtime/Val",
        JObject::null(),
    )?;
    let caller = ReentrantLock::new(caller);
    let caller_ptr = &caller as *const ReentrantLock<Caller<_>> as jlong;

    let call_result = env.call_static_method(
        "io/github/kawamuray/wasmtime/Func",
        "invokeTrampoline",
        "(JI[Lio/github/kawamuray/wasmtime/Val;[Lio/github/kawamuray/wasmtime/Val;)V",
        &[
            caller_ptr.into(),
            index.into(),
            (&unsafe { JObject::from_raw(jparams) }).into(),
            (&unsafe { JObject::from_raw(jresults.as_raw()) }).into(),
        ],
    );
    if let Err(e) = call_result {
        match e {
            jni::errors::Error::JavaException => {
                let throwable = env.exception_occurred()?;
                env.exception_clear()?;
                let wasm_error = wfuncerror::from_java(&mut env, throwable)?;
                return Ok(Err(wasm_error));
            }
            _ => return Err(e.into()),
        }
    }

    // Fill java results value into Rust's
    let mut i = 0;
    let mut iter = utils::JavaArrayIter::new(&mut env, jresults.into_raw())?;
    while let Some(rval) = iter.next(&mut env) {
        let rval = rval?;
        if !rval.is_null() {
            results[i] = wval::from_java(&mut env, rval)?;
        }
        i += 1;
    }

    Ok(Ok(()))
}
