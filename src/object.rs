use jni::objects::{JObject, JValue, JMethodID, JClass};
use jni::JNIEnv;
use jni::errors::Result;
use crate::class::Class;

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
        Ok(Self::new(env.new_object_unchecked(Class::Long(env)?.0, Self::get_constructor(env, Class::Long(env)?, "(J)V")?, &[JValue::Long(l)])?, Class::Long(env)?))
    }

    /// Create a new java.lang.Integer
    pub fn new_Integer(env: &JNIEnv<'a>, i: i32) -> Result<Self> {
        Ok(Self::new(env.new_object_unchecked(Class::Integer(env)?.0, Self::get_constructor(env, Class::Integer(env)?, "(I)V")?, &[JValue::Int(i)])?, Class::Integer(env)?))
    }

    /// Create a new java.lang.Float
    pub fn new_Float(env: &JNIEnv<'a>, f: f32) -> Result<Self> {
        Ok(Self::new(env.new_object_unchecked(Class::Float(env)?.0, Self::get_constructor(env, Class::Float(env)?, "(F)V")?, &[JValue::Float(f)])?, Class::Float(env)?))
    }

    /// Create a new java.lang.Double
    pub fn new_Double(env: &JNIEnv<'a>, d: f64) -> Result<Self> {
        Ok(Self::new(env.new_object_unchecked(Class::Double(env)?.0, Self::get_constructor(env, Class::Double(env)?, "(D)V")?, &[JValue::Double(d)])?, Class::Float(env)?))
    }

    /// Create a new java.lang.Boolean
    pub fn new_Boolean(env: &JNIEnv<'a>, b: bool) -> Result<Self> {
        let int_val = if b { 1 } else { 0 };
        Ok(Self::new(env.new_object_unchecked(Class::Boolean(env)?.0, Self::get_constructor(env, Class::Boolean(env)?, "(Z)V")?, &[JValue::Bool(int_val)])?, Class::Boolean(env)?))
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

    /// Call java.object.Object#getClass() on the current Object
    pub fn get_class_of_self(&self, env: &JNIEnv<'a>) -> Result<Class<'a>> {
        Self::get_class(self, env)
    }

    /// Call java.object.Object#getClass() on `obj`
    pub fn get_class(obj: &Object<'a>, env: &JNIEnv<'a>) -> Result<Class<'a>> {
        let class_object = env.call_method(obj.obj, "getClass", "()Ljava/lang/Class;", &[])?.l()?;
        let class_name = Class(JClass::from(class_object)).get_name(env)?;
        Ok(Class::for_name(env, &class_name)?)
    }

    /// Check if the current Object is an instanceof the provided Class
    pub fn instance_of_class(&self, env: &JNIEnv<'a>, class: &Class) -> Result<bool> {
        env.is_instance_of(self.obj, class.0)
    }

    /// Check if the current Object is of the same instance as the other Object
    pub fn instance_of_same_object(&self, env: &JNIEnv<'a>, other: &Self) -> Result<bool> {
        env.is_instance_of(self.obj, other.class.0)
    }

    /// Check if the current Object is equal to another Object.
    pub fn equals(&self, env: &JNIEnv<'a>, other: &Object<'a>) -> Result<bool> {
        let equals = env.call_method(self.obj, "equals", "(Ljava/lang/Object;)Z", &[other.into()])?;
        Ok(equals.z()?)
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
        let rstring: String = env.get_string(JString::from(jstring.obj)).unwrap().into();
        assert_eq!("Foo", rstring.as_str());
    }

    #[test]
    fn new_Byte() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jByte = Object::new_Byte(&env, 0x1).unwrap();
        let jbyte = env.call_method(jByte.obj, "byteValue", "()B", &[]).unwrap().b().unwrap();
        assert_eq!(0x1, jbyte);
    }

    #[test]
    fn new_Long() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jLong = Object::new_Long(&env, 10).unwrap();
        let jlong = env.call_method(jLong.obj, "longValue", "()J", &[]).unwrap().j().unwrap();
        assert_eq!(10, jlong);
    }

    #[test]
    fn new_Integer() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jInteger = Object::new_Integer(&env, 10).unwrap();
        let jint = env.call_method(jInteger.obj, "intValue", "()I", &[]).unwrap().i().unwrap();
        assert_eq!(10, jint);
    }

    #[test]
    fn new_Float() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jFloat = Object::new_Float(&env, 10.5).unwrap();
        let jfloat = env.call_method(jFloat.obj, "floatValue", "()F", &[]).unwrap().f().unwrap();
        assert_eq!(10.5, jfloat);
    }

    #[test]
    fn new_Double() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jDouble = Object::new_Double(&env, 10.5).unwrap();
        let jdouble = env.call_method(jDouble.obj, "doubleValue", "()D", &[]).unwrap().d().unwrap();
        assert_eq!(10.5, jdouble);
    }

    #[test]
    fn new_Boolean() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let jBoolean = Object::new_Boolean(&env, true).unwrap();
        let jboolean = env.call_method(jBoolean.obj, "booleanValue", "()Z", &[]).unwrap().z().unwrap();
        assert_eq!(true, jboolean);

        let jBoolean = Object::new_Boolean(&env, false).unwrap();
        let jboolean = env.call_method(jBoolean.obj, "booleanValue", "()Z", &[]).unwrap().z().unwrap();
        assert_eq!(false, jboolean);
    }

    #[test]
    fn new_Array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let boolean = &[Object::new_Boolean(&env, true).unwrap(), Object::new_Boolean(&env, false).unwrap()];
        let boolean_array = Object::new_array(&env, Class::Boolean(&env).unwrap(), boolean).unwrap();

        let size = env.get_array_length(boolean_array.obj.into_inner()).unwrap();
        assert_eq!(2, size);

        let zeroth_element = env.get_object_array_element(boolean_array.obj.into_inner(), 0i32).unwrap();
        let bool_value = env.call_method(zeroth_element, "booleanValue", "()Z", &[]).unwrap().z().unwrap();

        assert_eq!(true, bool_value);
    }

    #[test]
    fn get_class() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let string = Object::new_string(&env, "Foo").unwrap();
        assert!(string.instance_of_class(&env, &string.get_class_of_self(&env).unwrap()).unwrap());
    }

    #[test]
    fn instance_of() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let string = Object::new_string(&env, "Foo").unwrap();

        assert!(string.instance_of_class(&env, &Class::String(&env).unwrap()).unwrap());
        assert!(string.instance_of_same_object(&env, &string).unwrap());
    }
}