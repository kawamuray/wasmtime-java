use super::JniModule;
use crate::errors::{self, Result};
use crate::wval::types_into_java_array;
use crate::{interop, utils, wmut, wval};
use jni::objects::{JByteArray, JClass, JObject, JString};
use jni::sys::{jbyteArray, jint, jlong, jobjectArray};
use jni::JNIEnv;
use wasmtime::{Engine, ExternType, Module};

pub(super) struct JniModuleImpl;

const OBJECT_CLASS: &'static str = "java/lang/Object";
pub const IMPORT_TYPE_CLASS: &'static str = "io/github/kawamuray/wasmtime/ImportType$Type";

impl<'a> JniModule<'a> for JniModuleImpl {
    type Error = errors::Error;

    fn dispose(env: &mut JNIEnv<'a>, this: JObject<'a>) -> Result<(), Self::Error> {
        interop::dispose_inner::<Module>(env, &this)?;
        Ok(())
    }

    fn imports(
        env: &mut JNIEnv<'a>,
        this: JObject<'a>,
    ) -> std::result::Result<jobjectArray, Self::Error> {
        const STRING_CLASS: &str = "java/lang/String";
        const IMPORT_TYPE: &str = "io/github/kawamuray/wasmtime/ImportType";

        let module = interop::get_inner::<Module>(env, &this)?;
        let it = module.imports();
        let mut imports = Vec::with_capacity(it.len());
        for obj in it {
            let module = obj.module();
            let (ty, ty_obj) = match obj.ty() {
                ExternType::Func(func) => {
                    let results = types_into_java_array(env, func.results())?;
                    let params = types_into_java_array(env, func.params())?;

                    (
                        into_java_import_type(env, "FUNC")?,
                        env.new_object(
                            "io/github/kawamuray/wasmtime/FuncType",
                            format!("([L{};[L{};)V", wval::VAL_TYPE, wval::VAL_TYPE),
                            &[
                                (&unsafe { JObject::from_raw(params) }).into(),
                                (&unsafe { JObject::from_raw(results) }).into(),
                            ],
                        )?,
                    )
                }
                ExternType::Global(global) => {
                    let java_ty = wval::type_into_java(env, global.content().to_owned())?;
                    let java_muta = wmut::mutability_into_java(env, global.mutability())?;
                    (
                        into_java_import_type(env, "GLOBAL")?,
                        env.new_object(
                            "io/github/kawamuray/wasmtime/GlobalType",
                            format!("(L{};L{};)V", wval::VAL_TYPE, wmut::MUT_TYPE),
                            &[(&java_ty).into(), (&(java_muta)).into()],
                        )?,
                    )
                }
                ExternType::Table(tab) => {
                    const TABLE_TYPE: &str = "io/github/kawamuray/wasmtime/TableType";
                    let val = wval::type_into_java(env, tab.element().to_owned())?;
                    let table = env.new_object(
                        TABLE_TYPE,
                        format!("(L{};II)V", wval::VAL_TYPE),
                        &[
                            (&val).into(),
                            (tab.minimum() as jint).into(),
                            tab.maximum().map(|v| v as jint).unwrap_or(-1).into(),
                        ],
                    );

                    (into_java_import_type(env, "TABLE")?, table?)
                }
                ExternType::Memory(mem) => {
                    const MEMORY_TYPE: &str = "io/github/kawamuray/wasmtime/MemoryType";
                    let mem = env.new_object(
                        MEMORY_TYPE,
                        "(JJZ)V",
                        &[
                            (mem.minimum() as jlong).into(),
                            mem.maximum().map(|v| v as jlong).unwrap_or(-1).into(),
                            mem.is_64().into(),
                        ],
                    );
                    (into_java_import_type(env, "MEMORY")?, mem?)
                }
            };

            let import = env.new_object(
                IMPORT_TYPE,
                format!(
                    "(L{};L{};L{};L{};)V",
                    IMPORT_TYPE_CLASS, OBJECT_CLASS, STRING_CLASS, STRING_CLASS
                ),
                &[
                    (&ty).into(),
                    (&ty_obj).into(),
                    (&(env.new_string(module)?)).into(),
                    (&(env.new_string(obj.name())?)).into(),
                ],
            )?;

            imports.push(import);
        }

        Ok(utils::into_java_array(env, IMPORT_TYPE, imports)?)
    }

    fn new_module(
        env: &mut JNIEnv<'a>,
        _clazz: JClass<'a>,
        engine_ptr: jlong,
        bytes: jbyteArray,
    ) -> Result<jlong, Self::Error> {
        let bytes = env.convert_byte_array(unsafe { JByteArray::from_raw(bytes) })?;
        let module = Module::new(&*interop::ref_from_raw::<Engine>(engine_ptr)?, &bytes)?;
        Ok(interop::into_raw::<Module>(module))
    }

    fn new_from_file(
        env: &mut JNIEnv<'a>,
        _clazz: JClass<'a>,
        engine_ptr: jlong,
        file_name: JString<'a>,
    ) -> Result<jlong, Self::Error> {
        let filename = utils::get_string(env, &file_name)?;
        let module = Module::from_file(&*interop::ref_from_raw::<Engine>(engine_ptr)?, &filename)?;
        Ok(interop::into_raw::<Module>(module))
    }

    fn new_from_binary(
        env: &mut JNIEnv<'a>,
        _clazz: JClass<'a>,
        engine_ptr: jlong,
        bytes: jbyteArray,
    ) -> Result<jlong, Self::Error> {
        let bytes = env.convert_byte_array(unsafe { JByteArray::from_raw(bytes) })?;
        let module = Module::from_binary(&*interop::ref_from_raw::<Engine>(engine_ptr)?, &bytes)?;
        Ok(interop::into_raw::<Module>(module))
    }
}

pub fn into_java_import_type<'a>(
    env: &mut JNIEnv<'a>,
    ty: &'a str,
) -> jni::errors::Result<JObject<'a>> {
    env.get_static_field(IMPORT_TYPE_CLASS, ty, format!("L{};", IMPORT_TYPE_CLASS))?
        .l()
}
