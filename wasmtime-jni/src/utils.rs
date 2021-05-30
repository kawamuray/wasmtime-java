use crate::errors::{Error, Result};
use jni::objects::JObject;
use jni::sys::{jint, jobjectArray};
use jni::JNIEnv;

/// Shorthand of turning `JObject` into a Rust `String`.
pub fn get_string(env: &JNIEnv, obj: JObject) -> Result<String> {
    Ok(String::from(env.get_string(obj.into())?))
}

/// Shorthand of obtaining and turning `JObject` from a field into a Rust `String`.
pub fn get_string_field(env: &JNIEnv, obj: JObject, field: &str) -> Result<String> {
    let s = env.get_field(obj, field, "Ljava/lang/String;")?.l()?;
    get_string(env, s)
}

/// Shorthand of obtaining and turning `JObject` from a field into a Rust `Option<String>`.
pub fn get_optional_string_field(env: &JNIEnv, obj: JObject, field: &str) -> Result<Option<String>> {
    let optional = env.get_field(obj, field, "Ljava/util/Optional;")?.l()?;
    let is_present = env.call_method(optional, "isPresent", "()Z", &[])?.z()?;
    if is_present {
        let s = env.call_method(optional, "get", "()Ljava/lang/Object;", &[])?.l()?;
        Ok(Some(get_string(env, s)?))
    } else {
        Ok(None)
    }
}


/// Convert a Vec of JObjects into jobjectArray.
pub fn into_java_array(env: &JNIEnv, clazz: &str, vec: Vec<JObject>) -> Result<jobjectArray> {
    let array = env.new_object_array(vec.len() as jint, clazz, JObject::null())?;
    for (i, obj) in vec.into_iter().enumerate() {
        env.set_object_array_element(array, i as jint, obj)?;
    }
    Ok(array)
}

/// Given an object of enum type, return its "name" as Rust's `String`.
pub fn enum_name(env: &JNIEnv, obj: JObject) -> Result<String> {
    let name = env
        .call_method(obj, "name", "()Ljava/lang/String;", &[])?
        .l()?;
    Ok(String::from(env.get_string(name.into())?))
}

/// Iterate over java object array.
pub struct JavaArrayIter<'a> {
    env: &'a JNIEnv<'a>,
    array: jobjectArray,
    len: usize,
    cur: usize,
}

impl<'a> JavaArrayIter<'a> {
    pub fn new(env: &'a JNIEnv<'a>, array: jobjectArray) -> Result<Self> {
        let len = env.get_array_length(array)? as usize;
        Ok(Self {
            env,
            array,
            len,
            cur: 0,
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<'a> Iterator for JavaArrayIter<'a> {
    type Item = Result<JObject<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.len {
            return None;
        }
        let ret = self
            .env
            .get_object_array_element(self.array, self.cur as jint);
        self.cur += 1;
        Some(ret.map_err(Into::into))
    }
}
