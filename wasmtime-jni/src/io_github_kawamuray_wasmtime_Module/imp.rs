use super::JniModule;
use crate::errors::{self, Result};
use crate::wval::types_into_java_array;
use crate::{interop, utils, wmut, wval};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jbyteArray, jint, jlong, jobjectArray};
use jni::JNIEnv;
use wasmtime::{Engine, ExternType, Limits, Module};

pub(super) struct JniModuleImpl;

const OBJECT_CLASS: &'static str = "java/lang/Object";
const LIMIT_TYPE: &str = "io/github/kawamuray/wasmtime/MemoryType$Limit";
pub const IMPORT_TYPE_CLASS: &'static str = "io/github/kawamuray/wasmtime/ImportType$Type";
pub const EXPORT_TYPE_CLASS: &'static str = "io/github/kawamuray/wasmtime/ExportType$Type";
pub const STRING_CLASS: &str = "java/lang/String";

macro_rules! meow {
    ($into_type:ident, $ty:expr, $env:ident) => {{
        match $ty {
            ExternType::Func(func) => {
                let results = types_into_java_array($env, func.results());
                let params = types_into_java_array($env, func.params());

                (
                    $into_type($env, "FUNC"),
                    $env.new_object(
                        "io/github/kawamuray/wasmtime/FuncType",
                        format!("([L{};[L{};)V", wval::VAL_TYPE, wval::VAL_TYPE),
                        &[params?.into(), results?.into()],
                    ),
                )
            }
            ExternType::Global(global) => (
                $into_type($env, "GLOBAL"),
                $env.new_object(
                    "io/github/kawamuray/wasmtime/GlobalType",
                    format!("(L{};L{};)V", wval::VAL_TYPE, wmut::MUT_TYPE),
                    &[
                        wval::type_into_java($env, global.content().to_owned())?
                            .into_inner()
                            .into(),
                        wmut::mutability_into_java($env, global.mutability())?
                            .into_inner()
                            .into(),
                    ],
                ),
            ),
            ExternType::Table(tab) => {
                const TABLE_TYPE: &str = "io/github/kawamuray/wasmtime/TableType";
                let limit = limit_into_java($env, tab.limits());
                let val = wval::type_into_java($env, tab.element().to_owned());
                let table = $env.new_object(
                    TABLE_TYPE,
                    format!("(L{};L{};)V", wval::VAL_TYPE, LIMIT_TYPE),
                    &[val?.into_inner().into(), limit?.into_inner().into()],
                );

                ($into_type($env, "TABLE"), table)
            }
            ExternType::Memory(mem) => {
                const MEMORY_TYPE: &str = "io/github/kawamuray/wasmtime/MemoryType";
                let limit = limit_into_java($env, mem.limits());
                let mem = $env.new_object(
                    MEMORY_TYPE,
                    format!("(L{};)V", LIMIT_TYPE),
                    &[limit?.into_inner().into()],
                );

                ($into_type($env, "MEMORY"), mem)
            }
            // WebAssembly module-linking proposal
            ExternType::Instance(_) => ($into_type($env, "INSTANCE"), Ok(JObject::null())),
            // WebAssembly module-linking proposal
            ExternType::Module(_) => ($into_type($env, "MODULE"), Ok(JObject::null())),
        }
    }};
}

impl<'a> JniModule<'a> for JniModuleImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::dispose_inner::<Module>(&env, this)?;
        Ok(())
    }

    fn exports(env: &JNIEnv, this: JObject) -> std::result::Result<jobjectArray, Self::Error> {
        const EXPORT_TYPE: &str = "io/github/kawamuray/wasmtime/ExportType";
        let module = interop::get_inner::<Module>(env, this)?;
        let it = module.exports();
        let mut exports = Vec::with_capacity(it.len());
        for obj in it {
            //     let obj: ExportType = obj;
            let name = env.new_string(obj.name());
            let (ty, ty_obj) = meow!(into_java_export_type, obj.ty(), env);
            let export = env.new_object(
                EXPORT_TYPE,
                format!(
                    "(L{};L{};L{};)V",
                    EXPORT_TYPE_CLASS, OBJECT_CLASS, STRING_CLASS
                ),
                &[
                    ty?.into_inner().into(),
                    ty_obj?.into_inner().into(),
                    name?.into(),
                ],
            )?;
            exports.push(export);
        }
        Ok(utils::into_java_array(env, EXPORT_TYPE, exports)?)
    }

    fn imports(env: &JNIEnv, this: JObject) -> std::result::Result<jobjectArray, Self::Error> {
        const IMPORT_TYPE: &str = "io/github/kawamuray/wasmtime/ImportType";

        let module = interop::get_inner::<Module>(env, this)?;
        let it = module.imports();
        let mut imports = Vec::with_capacity(it.len());
        for obj in it {
            let module = obj.module();
            let (ty, ty_obj) = meow!(into_java_import_type, obj.ty(), env);

            let name = obj.name().unwrap_or_else(|| "");
            let import = env.new_object(
                IMPORT_TYPE,
                format!(
                    "(L{};L{};L{};L{};)V",
                    IMPORT_TYPE_CLASS, OBJECT_CLASS, STRING_CLASS, STRING_CLASS
                ),
                &[
                    ty?.into_inner().into(),
                    ty_obj?.into_inner().into(),
                    env.new_string(module)?.into(),
                    env.new_string(name)?.into(),
                ],
            )?;

            imports.push(import);
        }

        Ok(utils::into_java_array(env, IMPORT_TYPE, imports)?)
    }

    fn new_module(
        env: &JNIEnv,
        _clazz: JClass,
        engine_ptr: jlong,
        bytes: jbyteArray,
    ) -> Result<jlong, Self::Error> {
        let bytes = env.convert_byte_array(bytes)?;
        let module = Module::new(&*interop::ref_from_raw::<Engine>(engine_ptr)?, &bytes)?;
        Ok(interop::into_raw::<Module>(module))
    }

    fn new_from_file(
        env: &JNIEnv,
        _clazz: JClass,
        engine_ptr: jlong,
        file_name: JString,
    ) -> Result<jlong, Self::Error> {
        let filename = utils::get_string(env, *file_name)?;
        let module = Module::from_file(&*interop::ref_from_raw::<Engine>(engine_ptr)?, &filename)?;
        Ok(interop::into_raw::<Module>(module))
    }

    fn new_from_binary(
        env: &JNIEnv,
        _clazz: JClass,
        engine_ptr: jlong,
        bytes: jbyteArray,
    ) -> Result<jlong, Self::Error> {
        let bytes = env.convert_byte_array(bytes)?;
        let module = Module::from_binary(&*interop::ref_from_raw::<Engine>(engine_ptr)?, &bytes)?;
        Ok(interop::into_raw::<Module>(module))
    }
}

fn limit_into_java<'a>(env: &'a JNIEnv, limits: &Limits) -> jni::errors::Result<JObject<'a>> {
    let min = limits.min() as jint;
    let max = match limits.max() {
        None => -1,
        Some(max) => max as jint,
    };
    env.new_object(LIMIT_TYPE, "(II)V", &[min.into(), max.into()])
}

pub fn into_java_export_type<'a>(env: &'a JNIEnv, ty: &'a str) -> jni::errors::Result<JObject<'a>> {
    env.get_static_field(EXPORT_TYPE_CLASS, ty, format!("L{};", EXPORT_TYPE_CLASS))?
        .l()
}

pub fn into_java_import_type<'a>(env: &'a JNIEnv, ty: &'a str) -> jni::errors::Result<JObject<'a>> {
    env.get_static_field(IMPORT_TYPE_CLASS, ty, format!("L{};", IMPORT_TYPE_CLASS))?
        .l()
}
