use jni::objects::{JClass, JObject};
use jni::errors::Result;
use jni::JNIEnv;
use crate::abstractions::string::JavaString;
use crate::object::Object;

/// Class describes java.lang.Class, with getters for often used classes from the Java standard library.
#[repr(transparent)]
pub struct Class<'a>(pub JClass<'a>);

impl<'a> From<JClass<'a>> for Class<'a> {
    fn from(a: JClass<'a>) -> Self {
        Self(a)
    }
}

impl<'a> Class<'a> {
    #![allow(non_snake_case)]

    /// java.lang.Byte
    pub fn Byte(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Byte")?))
    }

    /// java.lang.Boolean
    pub fn Boolean(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Boolean")?))
    }

    /// java.lang.Float
    pub fn Float(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Float")?))
    }

    /// java.lang.Integer
    pub fn Integer(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Integer")?))
    }

    /// java.lang.Double
    pub fn Double(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Double")?))
    }

    /// java.lang.Short
    pub fn Short(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Short")?))
    }

    /// java.lang.Character
    pub fn Character(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Character")?))
    }

    /// java.lang.Long
    pub fn Long(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Long")?))
    }

    /// java.lang.Object
    pub fn Object(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Object")?))
    }

    /// java.lang.Class
    pub fn Class(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Class")?))
    }

    /// java.lang.System
    pub fn System(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/System")?))
    }

    /// java.lang.CharSequence
    pub fn CharSequence(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/CharSequence")?))
    }

    /// java.lang.Math
    pub fn Math(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Math")?))
    }

    /// java.lang.Record
    pub fn Record(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Record")?))
    }

    /// java.lang.String
    pub fn String(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/String")?))
    }

    /// java.lang.StringBuilder
    pub fn StringBuilder(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/StringBuilder")?))
    }

    /// java.math.BigDecimal
    pub fn BigDecimal(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/math/BigDecimal")?))
    }

    /// java.math.BigInteger
    pub fn BigInteger(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/math/BigInteger")?))
    }

    /// java.lang.reflect.Array
    pub fn Array(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/reflect/Array")?))
    }

    /// java.lang.reflect.Field
    pub fn Field(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/reflect/Field")?))
    }

    /// java.lang.reflect.Method
    pub fn Method(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/reflect/Method")?))
    }

    /// java.lang.reflect.Constructor
    pub fn Constructor(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/reflect/Constructor")?))
    }

    /// java.util.Uuid
    pub fn Uuid(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/UUID")?))
    }

    /// java.util.Vector
    pub fn Vector(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Vector")?))
    }

    /// java.util.Map.Entry
    pub fn MapEntry(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Map$Entry")?))
    }

    /// java.util.Set
    pub fn Set(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Set")?))
    }

    /// java.util.Scanner
    pub fn Scanner(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Scanner")?))
    }

    /// java.util.Queue
    pub fn Queue(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Queue")?))
    }

    /// java.util.Random
    pub fn Random(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Random")?))
    }

    /// java.util.Properties
    pub fn Properties(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Properties")?))
    }

    /// java.util.Optional
    pub fn Optional(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Optional")?))
    }

    /// java.util.Objects
    pub fn Objects(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Objects")?))
    }

    /// java.util.Map
    pub fn Map(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Map")?))
    }

    /// java.util.Locale
    pub fn Locale(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Locale")?))
    }

    /// java.util.List
    pub fn List(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/List")?))
    }

    /// java.util.LinkedList
    pub fn LinkedList(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/LinkedList")?))
    }

    /// java.util.LinkedHashSet
    pub fn LinkedHashSet(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/LinkedHashSet")?))
    }

    /// java.util.LinkedHashMap
    pub fn LinkedHashMap(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/LinkedHashMap")?))
    }

    /// java.util.Iterator
    pub fn Iterator(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Iterator")?))
    }

    /// java.util.IdentityHashMap
    pub fn IdentityHashMap(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/IdentityHashMap")?))
    }

    /// java.util.HashTable
    pub fn Hashtable(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Hashtable")?))
    }

    /// java.util.HashSet
    pub fn HashSet(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/HashSet")?))
    }

    /// java.util.HashMap
    pub fn HashMap(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/HashMap")?))
    }

    /// java.util.EnumSet
    pub fn EnumSet(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/EnumSet")?))
    }

    /// java.util.EnumMap
    pub fn EnumMap(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/EnumMap")?))
    }

    /// java.util.Enumeration
    pub fn Enumeration(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Enumeration")?))
    }

    /// java.util.Comparator
    pub fn Comparator(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Comparator")?))
    }

    /// java.util.Comparators
    pub fn Comparators(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Comparators")?))
    }

    /// java.util.Collection
    pub fn Collection(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Collection")?))
    }

    /// java.util.Base64
    pub fn Base64(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Base64")?))
    }

    /// java.util.Arrays
    pub fn Arrays(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Arrays")?))
    }

    /// java.util.ArrayList
    pub fn ArrayList(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/ArrayList")?))
    }

    /// java.util.concurrent.atomic.AtomicInteger
    pub fn AtomicInteger(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicInteger")?))
    }

    /// java.util.concurrent.atomic.AtomicLong
    pub fn AtomicLong(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicLong")?))
    }

    /// java.util.concurrent.atomic.AtomicReference
    pub fn AtomicReference(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicReference")?))
    }

    /// java.util.concurrent.atomic.AtomicIntegerArray
    pub fn AtomicIntegerArray(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicIntegerArray")?))
    }

    /// java.util.concurrent.atomic.AtomicIntegerFieldUpdater
    pub fn AtomicIntegerFieldUpdater(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicIntegerFieldUpdater")?))
    }

    /// java.util.Data
    pub fn Date(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Date")?))
    }

    /// java.net.URI
    pub fn URI(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/net/URI")?))
    }

    /// java.util.concurrent.atomic.AtomicLongArray
    pub fn AtomicLongArray(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicLongArray")?))
    }

    /// java.util.concurrent.atomic.AtomicLongFieldUpdater
    pub fn AtomicLongFieldUpdater(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicLongFieldUpdater")?))
    }

    /// java.util.concurrent.atomic.AtomicMarkableReference
    pub fn AtomicMarkableReference(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicMarkableReference")?))
    }

    /// java.util.concurrent.atomic.AtomicReferenceArray
    pub fn AtomicReferenceArray(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicReferenceArray")?))
    }

    /// java.util.concurrent.atomic.AtomicReferenceFieldUpdater
    pub fn AtomicReferenceFieldUpdater(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicReferenceFieldUpdater")?))
    }

    /// java.util.concurrent.atomic.DoubleAccumulator
    pub fn DoubleAccumulator(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/DoubleAccumulator")?))
    }

    /// java.util.concurrent.atomic.DoubleAdder
    pub fn DoubleAdder(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/DoubleAdder")?))
    }

    /// java.util.concurrent.atomic.LongAccumulator
    pub fn LongAccumulator(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/LongAccumulator")?))
    }

    /// java.util.concurrent.atomic.LongAdder
    pub fn LongAdder(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/LongAdder")?))
    }

    /// java.util.concurrent.atomic.Striped64
    pub fn Striped64(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/Striped64")?))
    }

    /// java.util.concurrent.Future
    pub fn Future(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/Future")?))
    }

    /// java.util.concurrent.TimeUnit
    pub fn TimeUnit(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/TimeUnit")?))
    }

    /// java.util.regex.Pattern
    pub fn Pattern(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/regex/Pattern")?))
    }

    /// java.util.regex.Matcher
    pub fn Matcher(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/regex/Matcher")?))
    }

    /// java.time.Duration
    pub fn Duration(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/time/Duration")?))
    }

    /// java.time.Instant
    pub fn Instant(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/time/Instant")?))
    }

    /// java.io.File
    pub fn File(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/io/File")?))
    }

    /// java.io.InputStream
    pub fn InputStream(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/io/InputStream")?))
    }

    /// java.io.OutputStream
    pub fn OutputStream(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/io/OutputStream")?))
    }

    /// sun.misc.Unsafe
    pub fn Unsafe(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("sun/misc/Unsafe")?))
    }

    /// Find a class by it's Java name. Can be in the format:
    /// - `java/lang/String`
    /// - `java.lang.String`
    pub fn for_name(env: &JNIEnv<'a>, name: &str) -> Result<Self> {
        let name_patched = name.replace('.', "/");
        Ok(Self(env.find_class(&name_patched)?))
    }

    /// Get the array type of a class. E.g `java.lang.String` results in `java.lang.String[]`
    pub fn array_type(&self, env: &JNIEnv<'a>) -> Result<Self> {
        let arr = env.new_object_array(0, self.0, JObject::null())?;
        let arr_class = env.get_object_class(arr)?;
        Ok(Self(arr_class))
    }

    /// Check if the current Class can be safely cast to the the other Class. E.g `java.util.HashMap` is compatible with `java.util.Map`
    pub fn is_compatible(&self, env: &JNIEnv<'a>, other: &Class<'a>) -> Result<bool> {
        env.is_assignable_from(self.0, other.0)
    }

    /// Get the superclass of the current Class. Returns None if the current class has no superclass other than java.lang.Object, or if the current Class is an interface
    pub fn get_superclass(&self, env: &JNIEnv<'a>) -> Result<Option<Class<'a>>> {
        let maybe_superclass = env.get_superclass(self.0)?;
        match maybe_superclass.is_null() {
            true => Ok(None),
            false => Ok(Some(maybe_superclass.into()))
        }
    }

    /// Get the Class name. Invokes `Class#getName()`
    pub fn get_name(&self, env: &JNIEnv<'a>) -> Result<String> {
        let class_name_object = env.call_method(self.0, "getName", "()Ljava/lang/String;", &[])?.l()?;
        let class_name_string = JavaString::new(Object::new(class_name_object, Class::String(env)?));
        class_name_string.into_rust(env)
    }

    /// The Java primitive `int`
    pub fn int(env: &JNIEnv<'a>) -> Result<Self> {
        let int_class = env.call_static_method("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JavaString::from_rust(env, "int")?.into()])?;
        Ok(Self(JClass::from(int_class.l()?)))
    }

    /// The Java primitive `long`
    pub fn long(env: &JNIEnv<'a>) -> Result<Self> {
        let int_class = env.call_static_method("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JavaString::from_rust(env, "long")?.into()])?;
        Ok(Self(JClass::from(int_class.l()?)))
    }

    /// The Java primitive `byte`
    pub fn byte(env: &JNIEnv<'a>) -> Result<Self> {
        let int_class = env.call_static_method("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JavaString::from_rust(env, "byte")?.into()])?;
        Ok(Self(JClass::from(int_class.l()?)))
    }

    /// The Java primitive `boolean`
    pub fn boolean(env: &JNIEnv<'a>) -> Result<Self> {
        let int_class = env.call_static_method("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JavaString::from_rust(env, "boolean")?.into()])?;
        Ok(Self(JClass::from(int_class.l()?)))
    }

    /// The Java primitive `float`
    pub fn float(env: &JNIEnv<'a>) -> Result<Self> {
        let int_class = env.call_static_method("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JavaString::from_rust(env, "float")?.into()])?;
        Ok(Self(JClass::from(int_class.l()?)))
    }

    /// The Java primitive `double`
    pub fn double(env: &JNIEnv<'a>) -> Result<Self> {
        let int_class = env.call_static_method("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JavaString::from_rust(env, "double")?.into()])?;
        Ok(Self(JClass::from(int_class.l()?)))
    }

    /// The Java primitive `short`
    pub fn short(env: &JNIEnv<'a>) -> Result<Self> {
        let int_class = env.call_static_method("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JavaString::from_rust(env, "short")?.into()])?;
        Ok(Self(JClass::from(int_class.l()?)))
    }

    /// The Java primitive `char`
    pub fn char(env: &JNIEnv<'a>) -> Result<Self> {
        let int_class = env.call_static_method("java/lang/Class", "getPrimitiveClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JavaString::from_rust(env, "char")?.into()])?;
        Ok(Self(JClass::from(int_class.l()?)))
    }
}

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]
    use crate::test::*;
    use super::Class;
    use jni::errors::Result;

    #[test]
    fn Byte() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Byte(&env).is_ok());
    }

    #[test]
    fn Boolean() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Boolean(&env).is_ok());
    }

    #[test]
    fn Float() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Float(&env).is_ok());
    }

    #[test]
    fn Integer() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Integer(&env).is_ok());
    }

    #[test]
    fn Double() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Double(&env).is_ok());
    }

    #[test]
    fn Short() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Short(&env).is_ok());
    }

    #[test]
    fn Character() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Character(&env).is_ok());
    }

    #[test]
    fn Long() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Long(&env).is_ok());
    }

    #[test]
    fn Object() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Object(&env).is_ok());
    }

    #[test]
    fn Class_() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Class(&env).is_ok());
    }

    #[test]
    fn System() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::System(&env).is_ok());
    }

    #[test]
    fn CharSequence() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::CharSequence(&env).is_ok());
    }

    #[test]
    fn Math() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Math(&env).is_ok());
    }

    #[test]
    fn Record() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Record(&env).is_ok());
    }

    #[test]
    fn String() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::String(&env).is_ok());
    }

    #[test]
    fn StringBuilder() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::StringBuilder(&env).is_ok());
    }

    #[test]
    fn BigDecimal() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::BigDecimal(&env).is_ok());
    }

    #[test]
    fn BigInteger() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::BigInteger(&env).is_ok());
    }

    #[test]
    fn Array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Array(&env).is_ok());
    }

    #[test]
    fn Field() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Field(&env).is_ok());
    }

    #[test]
    fn Method() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Method(&env).is_ok());
    }

    #[test]
    fn Constructor() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Constructor(&env).is_ok());
    }

    #[test]
    fn Uuid() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Uuid(&env).is_ok());
    }

    #[test]
    fn Vector() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Vector(&env).is_ok());
    }

    #[test]
    fn MapEntry() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::MapEntry(&env).is_ok());
    }

    #[test]
    fn Set() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Set(&env).is_ok());
    }

    #[test]
    fn Scanner() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Scanner(&env).is_ok());
    }

    #[test]
    fn Queue() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Queue(&env).is_ok());
    }
    #[test]
    fn Random() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Random(&env).is_ok());
    }

    #[test]
    fn Properties() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Properties(&env).is_ok());
    }

    #[test]
    fn Optional() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Optional(&env).is_ok());
    }

    #[test]
    fn Objects() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Objects(&env).is_ok());
    }

    #[test]
    fn Map() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Map(&env).is_ok());
    }

    #[test]
    fn Locale() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Locale(&env).is_ok());
    }

    #[test]
    fn List() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::List(&env).is_ok());
    }

    #[test]
    fn LinkedList() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::LinkedList(&env).is_ok());
    }

    #[test]
    fn LinkedHashSet() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::LinkedHashSet(&env).is_ok());
    }

    #[test]
    fn LinkedHashMap() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::LinkedHashMap(&env).is_ok());
    }

    #[test]
    fn Iterator() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Iterator(&env).is_ok());
    }

    #[test]
    fn IdentityHashMap() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::IdentityHashMap(&env).is_ok());
    }

    #[test]
    fn Hashtable() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Hashtable(&env).is_ok());
    }

    #[test]
    fn HashSet() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::HashSet(&env).is_ok());
    }

    #[test]
    fn HashMap() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::HashMap(&env).is_ok());
    }

    #[test]
    fn EnumSet() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::EnumSet(&env).is_ok());
    }

    #[test]
    fn EnumMap() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::EnumMap(&env).is_ok());
    }

    #[test]
    fn Enumeration() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Enumeration(&env).is_ok());
    }

    #[test]
    fn Comparator() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Comparator(&env).is_ok());
    }

    #[test]
    fn Comparators() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Comparators(&env).is_ok());
    }

    #[test]
    fn Collection() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Collection(&env).is_ok());
    }

    #[test]
    fn Base64() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Base64(&env).is_ok());
    }

    #[test]
    fn Arrays() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Arrays(&env).is_ok());
    }

    #[test]
    fn ArrayList() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::ArrayList(&env).is_ok());
    }

    #[test]
    fn AtomicInteger() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicInteger(&env).is_ok());
    }

    #[test]
    fn AtomicLong() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicLong(&env).is_ok());
    }

    #[test]
    fn AtomicReference() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicReference(&env).is_ok());
    }

    #[test]
    fn AtomicIntegerArray() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicIntegerArray(&env).is_ok());
    }

    #[test]
    fn AtomicIntegerFieldUpdater() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicIntegerFieldUpdater(&env).is_ok());
    }

    #[test]
    fn AtomicLongArray() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicLongArray(&env).is_ok());
    }

    #[test]
    fn AtomicLongFieldUpdater() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicLongFieldUpdater(&env).is_ok());
    }

    #[test]
    fn AtomicMarkableReference() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicMarkableReference(&env).is_ok());
    }

    #[test]
    fn AtomicReferenceArray() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicReferenceArray(&env).is_ok());
    }

    #[test]
    fn AtomicReferenceFieldUpdater() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::AtomicReferenceFieldUpdater(&env).is_ok());
    }

    #[test]
    fn DoubleAccumulator() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::DoubleAccumulator(&env).is_ok());
    }

    #[test]
    fn DoubleAdder() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::DoubleAdder(&env).is_ok());
    }

    #[test]
    fn LongAccumulator() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::LongAccumulator(&env).is_ok());
    }

    #[test]
    fn LongAdder() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::LongAdder(&env).is_ok());
    }

    #[test]
    fn Striped64() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Striped64(&env).is_ok());
    }

    #[test]
    fn Future() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Future(&env).is_ok());
    }

    #[test]
    fn TimeUnit() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::TimeUnit(&env).is_ok());
    }

    #[test]
    fn Pattern() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Pattern(&env).is_ok());
    }

    #[test]
    fn Matcher() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Matcher(&env).is_ok());
    }

    #[test]
    fn Duration() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Duration(&env).is_ok());
    }

    #[test]
    fn Instant() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Instant(&env).is_ok());
    }

    #[test]
    fn File() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::File(&env).is_ok());
    }

    #[test]
    fn InputStream() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::InputStream(&env).is_ok());
    }

    #[test]
    fn OutputStream() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::OutputStream(&env).is_ok());
    }

    #[test]
    fn Unsafe() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Unsafe(&env).is_ok());
    }

    #[test]
    fn Date() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::Date(&env).is_ok());
    }

    #[test]
    fn URI() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::URI(&env).is_ok());
    }

    #[test]
    fn for_name_slash() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::for_name(&env, "java/lang/String").is_ok())
    }

    #[test]
    fn for_name_dot() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::for_name(&env,"java.lang.String").is_ok());
    }

    #[test]
    fn array_type() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::BigInteger(&env).unwrap().array_type(&env).is_ok());
    }

    #[test]
    fn is_compatible() -> Result<()> {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let hashmap = crate::abstractions::map::Map::new_HashMap(&env, Class::Object(&env)?, Class::Object(&env)?)?;

        let is_compat = hashmap.inner.class.is_compatible(&env, &Class::Map(&env)?)?;
        assert!(is_compat);
        Ok(())
    }

    #[test]
    fn int() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::int(&env).is_ok());
    }

    #[test]
    fn long() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::long(&env).is_ok());
    }

    #[test]
    fn float() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::float(&env).is_ok());
    }

    #[test]
    fn double() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::double(&env).is_ok());
    }

    #[test]
    fn byte() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::byte(&env).is_ok());
    }

    #[test]
    fn boolean() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::boolean(&env).is_ok());
    }

    #[test]
    fn char() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::char(&env).is_ok());
    }

    #[test]
    fn short() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(Class::short(&env).is_ok());
    }
}