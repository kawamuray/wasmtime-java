use crate::errors::{Error, Result};
use jni::objects::{JObject, JObjectArray};
use jni::sys::{jint, jobjectArray};
use jni::JNIEnv;

/// Shorthand of turning `JObject` into a Rust `String`.
pub fn get_string(env: &mut JNIEnv, obj: &JObject) -> Result<String> {
    Ok(String::from(env.get_string(obj.into())?))
}

/// Shorthand of obtaining and turning `JObject` from a field into a Rust `String`.
pub fn get_string_field(env: &mut JNIEnv, obj: &JObject, field: &str) -> Result<String> {
    let s = env.get_field(obj, field, "Ljava/lang/String;")?.l()?;
    get_string(env, &s)
}

/// Convert a Vec of JObjects into jobjectArray.
pub fn into_java_array(env: &mut JNIEnv, clazz: &str, vec: Vec<JObject>) -> Result<jobjectArray> {
    let array = env.new_object_array(vec.len() as jint, clazz, JObject::null())?;
    for (i, obj) in vec.into_iter().enumerate() {
        env.set_object_array_element(&array, i as jint, obj)?;
    }
    Ok(array.into_raw())
}

/// Given an object of enum type, return its "name" as Rust's `String`.
pub fn enum_name(env: &mut JNIEnv, obj: JObject) -> Result<String> {
    let name = env
        .call_method(obj, "name", "()Ljava/lang/String;", &[])?
        .l()?;
    let jstr = env.get_string((&name).into())?;
    Ok(String::from(jstr))
}

/// Iterate over java object array.
pub struct JavaArrayIter {
    array: jobjectArray,
    len: usize,
    cur: usize,
}

impl JavaArrayIter {
    pub fn new(env: &mut JNIEnv, array: jobjectArray) -> Result<Self> {
        let len = env.get_array_length(&unsafe { JObjectArray::from_raw(array) })? as usize;
        Ok(Self { array, len, cur: 0 })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn next<'a>(&mut self, env: &mut JNIEnv<'a>) -> Option<Result<JObject<'a>, Error>> {
        if self.cur >= self.len {
            return None;
        }
        let ret = env.get_object_array_element(
            &unsafe { JObjectArray::from_raw(self.array) },
            self.cur as jint,
        );
        self.cur += 1;
        Some(ret.map_err(Into::into))
    }
}
