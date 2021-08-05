use jni::objects::{JObject, JValue, JMethodID};
use jni::JNIEnv;
use jni::errors::Result;
use crate::class::Class;

#[repr(transparent)]
pub struct Object<'a>(pub JObject<'a>);

impl<'a> Object<'a> {
    #![allow(non_snake_case)]

    fn get_method_id(env: &JNIEnv<'a>, class: Class<'a>, sig: &str) -> Result<JMethodID<'a>>{
        env.get_method_id(class.0, "<init>", sig)
    }

    pub fn new_string(env: &JNIEnv<'a>, str: impl AsRef<str>) -> Result<Self> {
        Ok(Self(env.new_string(str.as_ref())?.into()))
    }

    pub fn new_Byte(env: &JNIEnv<'a>, b: u8) -> Result<Self> {
        Ok(Self(env.new_object_unchecked(Class::Byte(env)?.0, Self::get_method_id(env, Class::Byte(env)?, "(B)V")?, &[JValue::Byte(b as i8)])?))
    }

    pub fn new_Long(env: &JNIEnv<'a>, l: i64) -> Result<Self> {
        Ok(Self(env.new_object_unchecked(Class::Long(env)?.0, Self::get_method_id(env, Class::Byte(env)?, "(J)V")?, &[JValue::Long(l)])?))
    }

    pub fn new_Integer(env: &JNIEnv<'a>, i: i32) -> Result<Self> {
        Ok(Self(env.new_object_unchecked(Class::Integer(env)?.0, Self::get_method_id(env, Class::Byte(env)?, "(I)V")?, &[JValue::Int(i)])?))
    }

    pub fn new_Float(env: &JNIEnv<'a>, f: f32) -> Result<Self> {
        Ok(Self(env.new_object_unchecked(Class::Float(env)?.0, Self::get_method_id(env, Class::Byte(env)?, "(F)V")?, &[JValue::Float(f)])?))
    }

    pub fn new_Double(env: &JNIEnv<'a>, d: f64) -> Result<Self> {
        Ok(Self(env.new_object_unchecked(Class::Double(env)?.0, Self::get_method_id(env, Class::Byte(env)?, "(D)V")?, &[JValue::Double(d)])?))
    }

    pub fn new_Boolean(env: &JNIEnv<'a>, b: bool) -> Result<Self> {
        let int_val = if b { 1 } else { 0 };
        Ok(Self(env.new_object_unchecked(Class::Boolean(env)?.0, Self::get_method_id(env, Class::Byte(env)?, "(B)V")?, &[JValue::Bool(int_val)])?))
    }

    pub fn new_array(env: &JNIEnv<'a>, class: Class<'a>, data: &'a [Self]) -> Result<Self> {
        let arr = env.new_object_array(data.len() as i32, class.0, JObject::null())?;

        for i in 0..data.len() {
            let elem = data.get(i).unwrap();
            env.set_object_array_element(arr, i as i32, elem.0)?;
        }

        Ok(Self(JObject::from(arr)))
    }
}