use jni::objects::{JClass, JObject};
use jni::errors::Result;
use jni::JNIEnv;

#[repr(transparent)]
pub struct Class<'a>(pub JClass<'a>);

impl<'a> Class<'a> {
    #![allow(non_snake_case)]

    pub fn Byte(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Byte")?))
    }

    pub fn Boolean(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Boolean")?))
    }

    pub fn Float(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Float")?))
    }

    pub fn Integer(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Integer")?))
    }

    pub fn Double(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Double")?))
    }

    pub fn Short(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Short")?))
    }

    pub fn Character(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Character")?))
    }

    pub fn Long(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Long")?))
    }

    pub fn Object(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Object")?))
    }

    pub fn Class(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Class")?))
    }

    pub fn System(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/System")?))
    }

    pub fn CharSequence(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/CharSequence")?))
    }

    pub fn Math(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Math")?))
    }

    pub fn Record(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Record")?))
    }

    pub fn String(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/String")?))
    }

    pub fn StringBuilder(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/StringBuilder")?))
    }

    pub fn BigDecimal(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/math/BigDecimal")?))
    }

    pub fn BigInteger(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/math/BigInteger")?))
    }

    pub fn Array(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/reflect/Array")?))
    }

    pub fn Field(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/reflect/Field")?))
    }

    pub fn Method(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/reflect/Method")?))
    }

    pub fn Constructor(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/reflect/Constructor")?))
    }

    pub fn Uuid(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/UUID")?))
    }

    pub fn Vector(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Vector")?))
    }

    pub fn Set(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Set")?))
    }

    pub fn Scanner(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Scanner")?))
    }

    pub fn Queue(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Queue")?))
    }

    pub fn Random(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Scanner")?))
    }

    pub fn Properties(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Properties")?))
    }

    pub fn Optional(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/lang/Optional")?))
    }

    pub fn Objects(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Objects")?))
    }

    pub fn Map(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Map")?))
    }

    pub fn Locale(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Locale")?))
    }

    pub fn List(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/utl/List")?))
    }

    pub fn LinkedList(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/LinkedList")?))
    }

    pub fn LinkedHashSet(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/utl/LinkedHashSet")?))
    }

    pub fn LinkedHashMap(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/LinkedHashMap")?))
    }

    pub fn Iterator(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Iterator")?))
    }

    pub fn IdentityHashMap(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/IdentityHashMap")?))
    }

    pub fn Hashtable(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Hashtable")?))
    }

    pub fn HashSet(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/HashSet")?))
    }

    pub fn HashMap(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/HashMap")?))
    }

    pub fn EnumSet(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/EnumSet")?))
    }

    pub fn EnumMap(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/EnumMap")?))
    }

    pub fn Enumeration(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Enumeration")?))
    }

    pub fn Comparator(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Comparator")?))
    }

    pub fn Comparators(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Comparators")?))
    }

    pub fn Collection(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Collection")?))
    }

    pub fn Base64(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Base64")?))
    }

    pub fn Arrays(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/Arrays")?))
    }

    pub fn ArrayList(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/ArrayList")?))
    }

    pub fn AtomicInteger(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicInteger")?))
    }

    pub fn AtomicLong(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicLong")?))
    }

    pub fn AtomicReference(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicReference")?))
    }

    pub fn AtomicIntegerArray(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicIntegerArray")?))
    }

    pub fn AtomicIntegerFieldUpdater(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicIntegerFieldUpdater")?))
    }

    pub fn AtomicLongArray(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicLongArray")?))
    }

    pub fn AtomicLongFieldUpdater(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicLongFieldUpdater")?))
    }

    pub fn AtomicMarkableReference(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicMarkableReference")?))
    }

    pub fn AtomicReferenceArray(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicReferenceArray")?))
    }

    pub fn AtomicReferenceFieldUpdater(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/AtomicReferenceFieldUpdater")?))
    }

    pub fn DoubleAccumulator(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/DoubleAccumulator")?))
    }

    pub fn DoubleAdder(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/DoubleAdder")?))
    }

    pub fn LongAccumulator(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/LongAccumulator")?))
    }

    pub fn LongAdder(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/LongAdder")?))
    }

    pub fn Striped64(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/atomic/Striped64")?))
    }

    pub fn Future(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/Future")?))
    }

    pub fn TimeUnit(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/concurrent/TimeUnit")?))
    }

    pub fn Pattern(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/regex/Pattern")?))
    }

    pub fn Matcher(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/util/regex/Matcher")?))
    }

    pub fn Duration(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/time/Duration")?))
    }

    pub fn Instant(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/time/Instant")?))
    }

    pub fn File(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/io/File")?))
    }

    pub fn InputStream(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/io/InputStream")?))
    }

    pub fn OutputStream(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("java/io/OutputStream")?))
    }

    pub fn Unsafe(env: &JNIEnv<'a>) -> Result<Self> {
        Ok(Self(env.find_class("sun/misc/Unsafe")?))
    }

    pub fn array_type(&self, env: &JNIEnv<'a>) -> Result<Self> {
        let arr = env.new_object_array(0, self.0, JObject::null())?;
        let arr_class = env.get_object_class(arr)?;
        Ok(Self(arr_class))
    }
}