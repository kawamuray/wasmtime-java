use super::JniModule;
use crate::errors::{self, Result};
use crate::wval::types_into_java_array;
use crate::{interop, utils, wmut, wval};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jbyteArray, jint, jlong, jobjectArray};
use jni::JNIEnv;
use wasmtime::{Engine, ExternType, Module};

pub(super) struct JniModuleImpl;

const OBJECT_CLASS: &'static str = "java/lang/Object";
pub const IMPORT_TYPE_CLASS: &'static str = "io/github/kawamuray/wasmtime/ImportType$Type";

impl<'a> JniModule<'a> for JniModuleImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::dispose_inner::<Module>(&env, this)?;
        Ok(())
    }

    fn imports(env: &JNIEnv, this: JObject) -> std::result::Result<jobjectArray, Self::Error> {
        const STRING_CLASS: &str = "java/lang/String";
        const IMPORT_TYPE: &str = "io/github/kawamuray/wasmtime/ImportType";

        let module = interop::get_inner::<Module>(env, this)?;
        let it = module.imports();
        let mut imports = Vec::with_capacity(it.len());
        for obj in it {
            let module = obj.module();
            let (ty, ty_obj) = match obj.ty() {
                ExternType::Func(func) => {
                    let results = types_into_java_array(env, func.results());
                    let params = types_into_java_array(env, func.params());

                    (
                        into_java_import_type(env, "FUNC"),
                        env.new_object(
                            "io/github/kawamuray/wasmtime/FuncType",
                            format!("([L{};[L{};)V", wval::VAL_TYPE, wval::VAL_TYPE),
                            &[params?.into(), results?.into()],
                        ),
                    )
                }
                ExternType::Global(global) => (
                    into_java_import_type(env, "GLOBAL"),
                    env.new_object(
                        "io/github/kawamuray/wasmtime/GlobalType",
                        format!("(L{};L{};)V", wval::VAL_TYPE, wmut::MUT_TYPE),
                        &[
                            wval::type_into_java(env, global.content().to_owned())?
                                .into_inner()
                                .into(),
                            wmut::mutability_into_java(env, global.mutability())?
                                .into_inner()
                                .into(),
                        ],
                    ),
                ),
                ExternType::Table(_) => (into_java_import_type(env, "TABLE"), Ok(JObject::null())),
                ExternType::Memory(mem) => {
                    let limits = mem.limits();

                    const MEMORY_TYPE: &str = "io/github/kawamuray/wasmtime/MemoryType";
                    const LIMIT_TYPE: &str = "io/github/kawamuray/wasmtime/MemoryType$Limit";
                    let min = limits.min() as jint;
                    let max = match limits.max() {
                        None => -1,
                        Some(max) => max as jint,
                    };

                    let limit = env.new_object(LIMIT_TYPE, "(II)V", &[min.into(), max.into()]);
                    let mem = env.new_object(
                        MEMORY_TYPE,
                        format!("(L{};)V", LIMIT_TYPE),
                        &[limit?.into_inner().into()],
                    );

                    (into_java_import_type(env, "MEMORY"), mem)
                }
                // WebAssembly module-linking proposal
                ExternType::Instance(_) => {
                    (into_java_import_type(env, "INSTANCE"), Ok(JObject::null()))
                }
                // WebAssembly module-linking proposal
                ExternType::Module(_) => {
                    (into_java_import_type(env, "MODULE"), Ok(JObject::null()))
                }
            };

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

pub fn into_java_import_type<'a>(env: &'a JNIEnv, ty: &'a str) -> jni::errors::Result<JObject<'a>> {
    env.get_static_field(IMPORT_TYPE_CLASS, ty, format!("L{};", IMPORT_TYPE_CLASS))?
        .l()
}
