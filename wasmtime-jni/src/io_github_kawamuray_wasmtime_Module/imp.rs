use super::JniModule;
use crate::errors::{self, Result};
use crate::{interop, utils};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jbyteArray, jlong, jobjectArray};
use jni::JNIEnv;
use wasmtime::{Engine, Module};
use crate::wextern::{EXTERN_TYPE_CLASS, type_into_java};

pub(super) struct JniModuleImpl;

const OBJECT_CLASS: &'static str = "java/lang/Object";
pub const STRING_CLASS: &str = "java/lang/String";

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
            let name = env.new_string(obj.name());
            let (ty, ty_obj) = type_into_java(env, obj.ty())?;
            let export = env.new_object(
                EXPORT_TYPE,
                format!(
                    "(L{};L{};L{};)V",
                    EXTERN_TYPE_CLASS, OBJECT_CLASS, STRING_CLASS
                ),
                &[
                    ty.into_inner().into(),
                    ty_obj.into_inner().into(),
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
            let (ty, ty_obj) = type_into_java(env, obj.ty())?;

            let name = obj.name().unwrap_or_else(|| "");
            let import = env.new_object(
                IMPORT_TYPE,
                format!(
                    "(L{};L{};L{};L{};)V",
                    EXTERN_TYPE_CLASS, OBJECT_CLASS, STRING_CLASS, STRING_CLASS
                ),
                &[
                    ty.into_inner().into(),
                    ty_obj.into_inner().into(),
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
