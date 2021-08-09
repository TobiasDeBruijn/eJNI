use jni::objects::{JClass, JObject};
use jni::errors::Result;
use jni::JNIEnv;

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
        Ok(Self(env.find_class("java/utl/List")?))
    }

    /// java.util.LinkedList
    pub fn LinkedList(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/LinkedList")?))
    }

    /// java.util.LinkedHashSet
    pub fn LinkedHashSet(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/utl/LinkedHashSet")?))
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


}