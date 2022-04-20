use super::JniModule;
use crate::errors::{self, Result};
use crate::{interop, utils};
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jbyteArray, jint, jlong, jobjectArray, jsize};
use jni::JNIEnv;

use crate::types::Import;
use wasmtime::{Engine, ExternType, ImportType, Module};
use crate::utils::into_java_array;

pub(super) struct JniModuleImpl;

impl<'a> JniModule<'a> for JniModuleImpl {
    type Error = errors::Error;

    fn dispose(env: &JNIEnv, this: JObject) -> Result<(), Self::Error> {
        interop::dispose_inner::<Module>(&env, this)?;
        Ok(())
    }

    fn imports<'b>(
        env: &'b JNIEnv,
        this: JObject,
    ) -> std::result::Result<jobjectArray, Self::Error> {
        const IMPORT_TYPE: &str = "io/github/kawamuray/wasmtime/ImportType";
        let module = interop::get_inner::<Module>(env, this)?;
        let imports = module.imports();
        let arr = env.new_object_array(imports.len() as jsize, IMPORT_TYPE, JObject::null())?;

        for (i, obj ) in imports.enumerate() {
            let module = obj.module();
            let ty = {
                match obj.ty() {
                    ExternType::Func(_) => "FUNC",
                    ExternType::Global(_) => "GLOBAL",
                    ExternType::Table(_) => "TABLE",
                    ExternType::Memory(_) => "MEMORY",
                    ExternType::Instance(_) => "INSTANCE",
                    ExternType::Module(_) => "MODULE",
                }
            };
            let name = obj.name().unwrap_or_else(|| "");

            let obj = Import {
                module: String::from(module),
                name: String::from(name),
                ty,
            };

            let import_type =
                env.new_object(IMPORT_TYPE, "(J)V", &[interop::into_raw(obj).into()])?;

            env.set_object_array_element(arr, i as jint, import_type)?;
        }

        Ok(arr)
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
