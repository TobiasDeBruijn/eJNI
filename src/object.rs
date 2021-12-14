use jni::objects::{JObject, JValue, JMethodID, JClass};
use jni::JNIEnv;
use jni::errors::Result;
use jni::signature::{JavaType, Primitive};
use crate::class::Class;
use jni::sys::_jobject;
use thiserror::Error;

/// Describes a Java Object
#[derive(Clone)]
pub struct Object<'a> {
    pub inner:  JObject<'a>,
    pub class:  Class<'a>,
    env:        &'a JNIEnv<'a>
}

#[allow(clippy::from_over_into)]
impl<'a> Into<JValue<'a>> for Object<'a> {
    fn into(self) -> JValue<'a> {
        JValue::Object(self.inner)
    }
}
#[allow(clippy::from_over_into)]
impl<'a> Into<JValue<'a>> for &Object<'a> {
    fn into(self) -> JValue<'a> {
        JValue::Object(self.inner)
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<*mut _jobject> for Object<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.into_inner()
    }
}

#[derive(Debug, Error)]
pub enum PrimitiveError<'a> {
    #[error("JNI Error: {0}")]
    Jni(#[from] jni::errors::Error),
    #[error("Expected {0:?}, but found {0:?}")]
    ClassMismatch(Class<'a>, Class<'a>)
}

pub type PrimitiveResult<'a, T> = std::result::Result<T, PrimitiveError<'a>>;

macro_rules! assert_same_class {
    ($a:expr, $b:expr) => {
        if $a.get_name()?.ne(&$b.get_name()?) {
            return Err(PrimitiveError::ClassMismatch($b, $a.clone()));
        }
    }
}

impl<'a> Object<'a> {
    /// Create a new Object wrapper. The caller must guarantee that the provided Object is of the same Class as the provided Class
    pub fn new(env: &'a JNIEnv<'a>, obj: JObject<'a>, class: Class<'a>) -> Self {
        Self {
            inner: obj,
            class,
            env
        }
    }

    /// Get a constructor
    fn get_constructor(env: &'a JNIEnv<'a>, class: Class<'a>, sig: &str) -> Result<JMethodID<'a>>{
        env.get_method_id(class.class, "<init>", sig)
    }

    /// Create a new java.lang.String
    pub fn new_string(env: &'a JNIEnv<'a>, str: impl AsRef<str>) -> Result<Self> {
        Ok(Self::new(env, env.new_string(str.as_ref())?.into(), Class::String(env)?))
    }

    /// Create a new java.lang.Byte
    pub fn new_byte_object(env: &'a JNIEnv<'a>, b: u8) -> Result<Self> {
        Ok(Self::new(env, env.new_object_unchecked(Class::Byte(env)?.class, Self::get_constructor(env, Class::Byte(env)?, "(B)V")?, &[JValue::Byte(b as i8)])?, Class::Byte(env)?))
    }

    /// Create a new java.lang.Long
    pub fn new_long_object(env: &'a JNIEnv<'a>, l: i64) -> Result<Self> {
        Ok(Self::new(env, env.new_object_unchecked(Class::Long(env)?.class, Self::get_constructor(env, Class::Long(env)?, "(J)V")?, &[JValue::Long(l)])?, Class::Long(env)?))
    }

    /// Create a new java.lang.Integer
    pub fn new_integer_object(env: &'a JNIEnv<'a>, i: i32) -> Result<Self> {
        Ok(Self::new(env, env.new_object_unchecked(Class::Integer(env)?.class, Self::get_constructor(env, Class::Integer(env)?, "(I)V")?, &[JValue::Int(i)])?, Class::Integer(env)?))
    }

    /// Create a new java.lang.Float
    pub fn new_float_object(env: &'a JNIEnv<'a>, f: f32) -> Result<Self> {
        Ok(Self::new(env, env.new_object_unchecked(Class::Float(env)?.class, Self::get_constructor(env, Class::Float(env)?, "(F)V")?, &[JValue::Float(f)])?, Class::Float(env)?))
    }

    /// Create a new java.lang.Double
    pub fn new_double_object(env: &'a JNIEnv<'a>, d: f64) -> Result<Self> {
        Ok(Self::new(env, env.new_object_unchecked(Class::Double(env)?.class, Self::get_constructor(env, Class::Double(env)?, "(D)V")?, &[JValue::Double(d)])?, Class::Double(env)?))
    }

    /// Create a new java.lang.Boolean
    pub fn new_boolean_object(env: &'a JNIEnv<'a>, b: bool) -> Result<Self> {
        let int_val = if b { 1 } else { 0 };
        Ok(Self::new(env, env.new_object_unchecked(Class::Boolean(env)?.class, Self::get_constructor(env, Class::Boolean(env)?, "(Z)V")?, &[JValue::Bool(int_val)])?, Class::Boolean(env)?))
    }

    /// Create a new java.lang.Character
    pub fn new_character_object(env: &'a JNIEnv<'a>, c: u16) -> Result<Self> {
        Ok(Self::new(env, env.new_object_unchecked(Class::Character(env)?.class, Self::get_constructor(env, Class::Character(env)?, "(C)V")?, &[JValue::Char(c)])?, Class::Character(env)?))
    }

    /// Create a new java.lang.Short
    pub fn new_short_object(env: &'a JNIEnv<'a>, s: i16) -> Result<Self> {
        Ok(Self::new(env, env.new_object_unchecked(Class::Short(env)?.class, Self::get_constructor(env, Class::Short(env)?, "(S)V")?, &[JValue::Short(s)])?, Class::Short(env)?))
    }

    /// Create an array from a slice of Objects. The caller must guarantee that all Objects contained in the slice are of the same Class as the provided Class
    pub fn new_array(env: &'a JNIEnv<'a>, class: Class<'a>, data: &'a [Self]) -> Result<Self> {
        let arr = env.new_object_array(data.len() as i32, class.class, JObject::null())?;

        for i in 0..data.len() {
            let elem = data.get(i).unwrap();
            env.set_object_array_element(arr, i as i32, elem.inner)?;
        }

        Ok(Self::new(env, JObject::from(arr), class.array_type(env)?))
    }

    /// Call java.object.Object#getClass() on the current Object
    pub fn get_class_of_self(&self) -> Result<Class<'a>> {
        Self::get_class(self, self.env)
    }

    /// Get the byte value from this Object. The Object must be of type java.lang.Byte
    pub fn get_byte(&self) -> PrimitiveResult<u8> {
        assert_same_class!(self.class, Class::Byte(self.env)?);

        let method = self.env.get_method_id("java/lang/Byte", "byteValue", "()B")?;
        let value = self.env.call_method_unchecked(self.inner, method, JavaType::Primitive(Primitive::Byte), &[])?;
        Ok(value.b()? as u8)
    }

    /// Get the long value from this Object. The Object must be of type java.lang.Long
    pub fn get_long(&self) -> PrimitiveResult<i64> {
        assert_same_class!(self.class, Class::Long(self.env)?);

        let method = self.env.get_method_id("java/lang/Long", "longValue", "()J")?;
        let value = self.env.call_method_unchecked(self.inner, method, JavaType::Primitive(Primitive::Long), &[])?;
        Ok(value.j()?)
    }

    /// Get the int value from this Object. The Object must be of type java.lang.Integer
    pub fn get_integer(&self) -> PrimitiveResult<i32> {
        assert_same_class!(self.class, Class::Integer(self.env)?);

        let method = self.env.get_method_id("java/lang/Integer", "intValue", "()I")?;
        let value = self.env.call_method_unchecked(self.inner, method, JavaType::Primitive(Primitive::Int), &[])?;
        Ok(value.i()?)
    }

    /// Get the float value from this Object. The Object must be of type java.lang.Float
    pub fn get_float(&self) -> PrimitiveResult<f32> {
        assert_same_class!(self.class, Class::Float(self.env)?);

        let method = self.env.get_method_id("java/lang/Float", "floatValue", "()F")?;
        let value = self.env.call_method_unchecked(self.inner, method, JavaType::Primitive(Primitive::Float), &[])?;
        Ok(value.f()?)
    }


    /// Get the double value from this Object. The Object must be of type java.lang.Double
    pub fn get_double(&self) -> PrimitiveResult<f64> {
        assert_same_class!(self.class, Class::Double(self.env)?);

        let method = self.env.get_method_id("java/lang/Double", "doubleValue", "()D")?;
        let value = self.env.call_method_unchecked(self.inner, method, JavaType::Primitive(Primitive::Double), &[])?;
        Ok(value.d()?)
    }


    /// Get the boolean value from this Object. The Object must be of type java.lang.Boolean
    pub fn get_boolean(&self) -> PrimitiveResult<bool> {
        assert_same_class!(self.class, Class::Boolean(self.env)?);

        let method = self.env.get_method_id("java/lang/Boolean", "booleanValue", "()Z")?;
        let value = self.env.call_method_unchecked(self.inner, method, JavaType::Primitive(Primitive::Boolean), &[])?;
        Ok(value.z()?)
    }

    /// Get the char value from this Object. The Object must be of type java.lang.Character
    /// Note that a Java character is two bytes. Java uses Unicode
    pub fn get_char(&self) -> PrimitiveResult<u16> {
        assert_same_class!(self.class, Class::Character(self.env)?);

        let method = self.env.get_method_id("java/lang/Character", "charValue", "()C")?;
        let value = self.env.call_method_unchecked(self.inner, method, JavaType::Primitive(Primitive::Char), &[])?;
        Ok(value.c()?)
    }

    /// Get the short value from this Object. The Object must be of type java.lang.Short
    pub fn get_short(&self) -> PrimitiveResult<i16> {
        assert_same_class!(self.class, Class::Short(self.env)?);

        let method = self.env.get_method_id("java/lang/Short", "shortValue", "()S")?;
        let value = self.env.call_method_unchecked(self.inner, method, JavaType::Primitive(Primitive::Short), &[])?;
        Ok(value.s()?)
    }

    /// Call java.object.Object#getClass() on `obj`
    pub fn get_class(obj: &Object<'a>, env: &'a JNIEnv<'a>) -> Result<Class<'a>> {
        let class_object = env.call_method(obj.inner, "getClass", "()Ljava/lang/Class;", &[])?.l()?;
        let class_name = Class::new(env, JClass::from(class_object)).get_name()?;
        Class::for_name(env, &class_name)
    }

    /// Check if the current Object is an instanceof the provided Class
    pub fn instance_of_class(&self, class: &Class) -> Result<bool> {
        self.env.is_instance_of(self.inner, class.class)
    }

    /// Check if the current Object is of the same instance as the other Object
    pub fn instance_of_same_object(&self, other: &Self) -> Result<bool> {
        self.env.is_instance_of(self.inner, other.class.class)
    }

    /// Check if the current Object is equal to another Object.
    pub fn equals(&self, other: &Object<'a>) -> Result<bool> {
        let equals = self.env.call_method(self.inner, "equals", "(Ljava/lang/Object;)Z", &[other.into()])?;
        equals.z()
    }
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use crate::test::JVM;
    use super::Object;
    use jni::objects::JString;
    use crate::class::Class;

    #[test]
    fn new_String() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jstring = Object::new_string(&env, "Foo").unwrap();
        let rstring: String = env.get_string(JString::from(jstring.inner)).unwrap().into();
        assert_eq!("Foo", rstring.as_str());
    }

    #[test]
    fn new_Byte() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jByte = Object::new_byte_object(&env, 0x1).unwrap();
        let jbyte = env.call_method(jByte.inner, "byteValue", "()B", &[]).unwrap().b().unwrap();
        assert_eq!(0x1, jbyte);
    }

    #[test]
    fn new_Long() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jLong = Object::new_long_object(&env, 10).unwrap();
        let jlong = env.call_method(jLong.inner, "longValue", "()J", &[]).unwrap().j().unwrap();
        assert_eq!(10, jlong);
    }

    #[test]
    fn new_Integer() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jInteger = Object::new_integer_object(&env, 10).unwrap();
        let jint = env.call_method(jInteger.inner, "intValue", "()I", &[]).unwrap().i().unwrap();
        assert_eq!(10, jint);
    }

    #[test]
    fn new_Float() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jFloat = Object::new_float_object(&env, 10.5).unwrap();
        let jfloat = env.call_method(jFloat.inner, "floatValue", "()F", &[]).unwrap().f().unwrap();
        assert_eq!(10.5, jfloat);
    }

    #[test]
    fn new_Double() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jDouble = Object::new_double_object(&env, 10.5).unwrap();
        let jdouble = env.call_method(jDouble.inner, "doubleValue", "()D", &[]).unwrap().d().unwrap();
        assert_eq!(10.5, jdouble);
    }

    #[test]
    fn new_Boolean() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jBoolean = Object::new_boolean_object(&env, true).unwrap();
        let jboolean = env.call_method(jBoolean.inner, "booleanValue", "()Z", &[]).unwrap().z().unwrap();
        assert_eq!(true, jboolean);

        let jBoolean = Object::new_boolean_object(&env, false).unwrap();
        let jboolean = env.call_method(jBoolean.inner, "booleanValue", "()Z", &[]).unwrap().z().unwrap();
        assert_eq!(false, jboolean);
    }

    #[test]
    fn new_Character() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let character = Object::new_character_object(&env, 123).unwrap();
        let jchar = env.call_method(character.inner, "charValue", "()C", &[]).unwrap().c().unwrap();
        assert_eq!(123, jchar);
    }

    #[test]
    fn new_Short() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let short = Object::new_short_object(&env, 123).unwrap();
        let jshort = env.call_method(short.inner, "shortValue", "()S", &[]).unwrap().s().unwrap();
        assert_eq!(123, jshort);
    }

    #[test]
    fn new_Array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let boolean = &[Object::new_boolean_object(&env, true).unwrap(), Object::new_boolean_object(&env, false).unwrap()];
        let boolean_array = Object::new_array(&env, Class::Boolean(&env).unwrap(), boolean).unwrap();

        let size = env.get_array_length(boolean_array.inner.into_inner()).unwrap();
        assert_eq!(2, size);

        let zeroth_element = env.get_object_array_element(boolean_array.inner.into_inner(), 0i32).unwrap();
        let bool_value = env.call_method(zeroth_element, "booleanValue", "()Z", &[]).unwrap().z().unwrap();

        assert_eq!(true, bool_value);
    }

    #[test]
    fn get_class() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let string = Object::new_string(&env, "Foo").unwrap();
        assert!(string.instance_of_class(&string.get_class_of_self().unwrap()).unwrap());
    }

    #[test]
    fn instance_of() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let string = Object::new_string(&env, "Foo").unwrap();

        assert!(string.instance_of_class(&Class::String(&env).unwrap()).unwrap());
        assert!(string.instance_of_same_object(&string).unwrap());
    }

    #[test]
    fn get_byte() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let byte = Object::new_byte_object(&env, 10).unwrap();
        let value = byte.get_byte().unwrap();

        assert_eq!(10, value);
    }

    #[test]
    fn get_long() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let long = Object::new_long_object(&env, 10).unwrap();
        let value = long.get_long().unwrap();

        assert_eq!(10, value);
    }

    #[test]
    fn get_integer() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let integer = Object::new_integer_object(&env, 10).unwrap();
        let value = integer.get_integer().unwrap();

        assert_eq!(10, value);
    }

    #[test]
    fn get_float() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let float = Object::new_float_object(&env, 10.0).unwrap();
        let value = float.get_float().unwrap();

        assert_eq!(10.0, value);
    }

    #[test]
    fn get_double() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let double = Object::new_double_object(&env, 10.0).unwrap();
        let value = double.get_double().unwrap();

        assert_eq!(10.0, value);
    }

    #[test]
    fn get_boolean() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let boolean = Object::new_boolean_object(&env, true).unwrap();
        let value = boolean.get_boolean().unwrap();

        assert_eq!(true, value);
    }

    #[test]
    fn get_char() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let character = Object::new_character_object(&env, 123).unwrap();
        let value = character.get_char().unwrap();

        assert_eq!(123, value);
    }

    #[test]
    fn get_short() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let short = Object::new_short_object(&env, 123).unwrap();
        let value = short.get_short().unwrap();

        assert_eq!(123, value);
    }

    #[test]
    fn get_wrong_class() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let integer = Object::new_integer_object(&env, 10).unwrap();
        let value = integer.get_float();
        assert!(value.is_err());
    }
}