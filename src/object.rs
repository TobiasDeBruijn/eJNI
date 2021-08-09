use jni::objects::{JObject, JValue, JMethodID};
use jni::JNIEnv;
use jni::errors::Result;
use crate::class::Class;
use crate::abstractions::string::JavaString;

/// Describes a Java Object
pub struct Object<'a> {
    pub obj:    JObject<'a>,
    pub class:  Class<'a>
}

impl<'a> Into<JValue<'a>> for Object<'a> {
    fn into(self) -> JValue<'a> {
        JValue::Object(self.obj)
    }
}

impl<'a> Into<JValue<'a>> for &Object<'a> {
    fn into(self) -> JValue<'a> {
        JValue::Object(*&self.obj)
    }
}

impl<'a> Object<'a> {
    #![allow(non_snake_case)]

    /// Create a new Object wrapper. The caller must guarantee that the provided Object is of the same Class as the provided Class
    pub fn new(obj: JObject<'a>, class: Class<'a>) -> Self {
        Self {
            obj,
            class
        }
    }

    /// Get a constructor
    fn get_constructor(env: &JNIEnv<'a>, class: Class<'a>, sig: &str) -> Result<JMethodID<'a>>{
        env.get_method_id(class.0, "<init>", sig)
    }

    /// Create a new java.lang.String
    pub fn new_string(env: &JNIEnv<'a>, str: impl AsRef<str>) -> Result<Self> {
        Ok(Self::new(env.new_string(str.as_ref())?.into(), Class::String(env)?))
    }

    /// Create a new java.lang.Byte
    pub fn new_Byte(env: &JNIEnv<'a>, b: u8) -> Result<Self> {
        Ok(Self::new(env.new_object_unchecked(Class::Byte(env)?.0, Self::get_constructor(env, Class::Byte(env)?, "(B)V")?, &[JValue::Byte(b as i8)])?, Class::Byte(env)?))
    }

    /// Create a new java.lang.Long
    pub fn new_Long(env: &JNIEnv<'a>, l: i64) -> Result<Self> {
        Ok(Self::new(env.new_object_unchecked(Class::Long(env)?.0, Self::get_constructor(env, Class::Byte(env)?, "(J)V")?, &[JValue::Long(l)])?, Class::Long(env)?))
    }

    /// Create a new java.lang.Integer
    pub fn new_Integer(env: &JNIEnv<'a>, i: i32) -> Result<Self> {
        Ok(Self::new(env.new_object_unchecked(Class::Integer(env)?.0, Self::get_constructor(env, Class::Byte(env)?, "(I)V")?, &[JValue::Int(i)])?, Class::Integer(env)?))
    }

    /// Create a new java.lang.Float
    pub fn new_Float(env: &JNIEnv<'a>, f: f32) -> Result<Self> {
        Ok(Self::new(env.new_object_unchecked(Class::Float(env)?.0, Self::get_constructor(env, Class::Byte(env)?, "(F)V")?, &[JValue::Float(f)])?, Class::Float(env)?))
    }

    /// Create a new java.lang.Double
    pub fn new_Double(env: &JNIEnv<'a>, d: f64) -> Result<Self> {
        Ok(Self::new(env.new_object_unchecked(Class::Double(env)?.0, Self::get_constructor(env, Class::Byte(env)?, "(D)V")?, &[JValue::Double(d)])?, Class::Float(env)?))
    }

    /// Create a new java.lang.Boolean
    pub fn new_Boolean(env: &JNIEnv<'a>, b: bool) -> Result<Self> {
        let int_val = if b { 1 } else { 0 };
        Ok(Self::new(env.new_object_unchecked(Class::Boolean(env)?.0, Self::get_constructor(env, Class::Byte(env)?, "(B)V")?, &[JValue::Bool(int_val)])?, Class::Boolean(env)?))
    }

    /// Create an array from a slice of Objects. The caller must guarantee that all Objects contained in the slice are of the same Class as the provided Class
    pub fn new_array(env: &JNIEnv<'a>, class: Class<'a>, data: &'a [Self]) -> Result<Self> {
        let arr = env.new_object_array(data.len() as i32, class.0, JObject::null())?;

        for i in 0..data.len() {
            let elem = data.get(i).unwrap();
            env.set_object_array_element(arr, i as i32, elem.obj)?;
        }

        Ok(Self::new(JObject::from(arr), class.array_type(env)?))
    }

    /// Call java.object.Class#getClass()
    pub fn get_class(&self, env: &JNIEnv<'a>) -> Result<Class<'a>> {
        let class_object = env.call_method(self.obj, "getClass", "()Ljava/lang/Class;", &[])?.l()?;
        let class_name_object = env.call_method(class_object, "getName", "()Ljava/lang/String;", &[])?.l()?;
        let class_name_string = JavaString::new(Object::new(class_name_object, Class::String(env)?));
        Ok(Class::for_name(env, &class_name_string.into_rust(env)?)?)
    }
}