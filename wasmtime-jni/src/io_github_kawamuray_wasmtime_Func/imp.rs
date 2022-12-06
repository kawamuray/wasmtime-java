use super::JniFunc;
use crate::errors::{self, Result};
use crate::interop::ReentrantLock;
use crate::store::StoreData;
use crate::{interop, utils, wtrap, wval};
use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong, jobjectArray};
use jni::{JNIEnv, JavaVM};
use wasmtime::{Caller, Func, FuncType, Store, Trap, Val};

pub(super) struct JniFuncImpl;

impl<'a> JniFunc<'a> for JniFuncImpl {
    type Error = errors::Error;

    fn new_func(
        env: &JNIEnv,
        _clazz: JClass,
        store_ptr: jlong,
        fn_type: JObject,
        index: jint,
    ) -> Result<jlong, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;

        let param_types = wval::types_from_java(
            &env,
            env.get_field(
                fn_type,
                "params",
                "[Lio/github/kawamuray/wasmtime/Val$Type;",
            )?
            .l()?
            .into_raw(),
        )?
        .into_boxed_slice();
        let result_types = wval::types_from_java(
            &env,
            env.get_field(
                fn_type,
                "results",
                "[Lio/github/kawamuray/wasmtime/Val$Type;",
            )?
            .l()?
            .into_raw(),
        )?
        .into_boxed_slice();
        let fn_type = FuncType::new(param_types.to_vec(), result_types.to_vec());

        let jvm = env.get_java_vm()?;
        let finalizer = FuncFinalizer {
            jvm: env.get_java_vm()?,
            index,
        };
        let func = Func::new(&mut *store, fn_type, move |caller, params, results| {
            let ret = invoke_trampoline(&jvm, index, caller, params, results, &finalizer);
            match ret {
                Ok(None) => Ok(()),
                Ok(Some(trap)) => Err(trap),
                Err(e) => Err(Trap::new(e.to_string())),
            }
        });

        Ok(interop::into_raw::<Func>(func))
    }

    fn native_call(
        env: &JNIEnv,
        this: JObject,
        store_ptr: jlong,
        args: jobjectArray,
    ) -> Result<jobjectArray, Self::Error> {
        let mut store = interop::ref_from_raw::<Store<StoreData>>(store_ptr)?;
        let func = interop::get_inner::<Func>(&env, this)?;

        let iter = utils::JavaArrayIter::new(env, args)?;
        let mut wasm_params = Vec::with_capacity(iter.len());
        for val in iter {
            let wasm_val = wval::from_java(env, val?)?;
            wasm_params.push(wasm_val);
        }
        let mut wasm_results =
            vec![unsafe { std::mem::zeroed() }; func.ty(&mut *store).results().len()];

        if let Err(e) = func.call(&mut *store, &wasm_params, &mut wasm_results) {
            return Err(match e.downcast::<Trap>() {
                Ok(trap) => errors::Error::WasmTrap(trap),
                Err(e) => e.into(),
            });
        }

        let java_results = wasm_results
            .into_iter()
            .map(|wasm_val| wval::into_java(env, wasm_val.clone()))
            .collect::<Result<Vec<_>, _>>()?;

        utils::into_java_array(env, "io/github/kawamuray/wasmtime/Val", java_results)
    }

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::dispose_inner::<Func>(&env, this)?;
        Ok(())
    }
}

struct FuncFinalizer {
    jvm: JavaVM,
    index: jint,
}

impl Drop for FuncFinalizer {
    fn drop(&mut self) {
        let env = self.jvm.attach_current_thread().unwrap();
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
) -> Result<Option<Trap>> {
    // TODO: this should be attach_current_thread_permanently?
    let env = jvm.attach_current_thread()?;

    // Convert Rust param values into Java's
    let jparams = utils::into_java_array(
        &env,
        "io/github/kawamuray/wasmtime/Val",
        params
            .into_iter()
            .map(|param| wval::into_java(&env, param.clone()))
            .collect::<Result<Vec<_>, _>>()?,
    )?;
    let jresults = env.new_object_array(
        results.len() as jint,
        "io/github/kawamuray/wasmtime/Val",
        JObject::null(),
    )?;
    let caller = ReentrantLock::new(caller);
    let caller_ptr = &caller as *const ReentrantLock<Caller<_>> as jlong;

    let trap = env
        .call_static_method(
            "io/github/kawamuray/wasmtime/Func",
            "invokeTrampoline",
            "(JI[Lio/github/kawamuray/wasmtime/Val;[Lio/github/kawamuray/wasmtime/Val;)Lio/github/kawamuray/wasmtime/Trap;",
            &[
                caller_ptr.into(),
                index.into(),
                unsafe { JObject::from_raw(jparams) }.into(),
                unsafe { JObject::from_raw(jresults) }.into(),
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
