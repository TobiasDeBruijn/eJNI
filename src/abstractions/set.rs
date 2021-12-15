use crate::object::Object;
use crate::class::Class;
use jni::JNIEnv;
use jni::errors::Result;
use jni::objects::JValue;
use crate::abstractions::iterator::Iterator;
use jni::sys::_jobject;

/// Wrapper around `java.util.Set`
pub struct Set<'a> {
    /// The Set itself
    pub inner:  Object<'a>,

    /// The Class contained in the Set
    pub class:  Class<'a>,

    env:        &'a JNIEnv<'a>
}

#[allow(clippy::from_over_into)]
impl<'a> Into<*mut _jobject> for Set<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.inner.into_inner()
    }
}

impl<'a> Drop for Set<'a> {
    fn drop(&mut self) {
        let _ = self.env.delete_local_ref(self.inner.inner);
    }
}

impl<'a> Set<'a> {
    /// Create a new Set. The caller must guarantee that the passed in Object implements Set and is not null.
    pub fn new(env: &'a JNIEnv<'a>, object: Object<'a>, class: Class<'a>) -> Self {
        Self {
            inner: object,
            class,
            env
        }
    }

    /// Constructs a new, empty set; the backing HashMap instance has default initial capacity (16) and load factor (0.75).
    pub fn hashset(env: &'a JNIEnv<'a>, v_class: Class<'a>) -> Result<Self> {
        let hashset = env.new_object("java/util/HashSet", "()V", &[])?;
        Ok(Self {
            inner: Object::new(env, hashset, Class::HashSet(env)?),
            class: v_class,
            env
        })
    }

    /// Constructs a new, empty set; the backing HashMap instance has the specified initial capacity and default load factor (0.75).
    pub fn hashset_with_capacity(env: &'a JNIEnv<'a>, v_class: Class<'a>, initial_capacity: i32) -> Result<Self> {
        let hashset = env.new_object("java/util/HashSet", "(I)V", &[JValue::Int(initial_capacity)])?;
        Ok(Self {
            inner: Object::new(env, hashset, Class::HashSet(env)?),
            class: v_class,
            env
        })
    }

    /// Constructs a new, empty set; the backing HashMap instance has the specified initial capacity and the specified load factor.
    pub fn hashset_with_capacity_and_load_factor(env: &'a JNIEnv<'a>, v_class: Class<'a>, initial_capacity: i32, load_factor: f32) -> Result<Self> {
        let hashset = env.new_object("java/util/HashSet", "(IF)V", &[JValue::Int(initial_capacity), JValue::Float(load_factor)])?;
        Ok(Self {
            inner: Object::new(env, hashset, Class::HashSet(env)?),
            class: v_class,
            env
        })
    }

    /// Returns the number of elements in this set (its cardinality).
    pub fn size(&self) -> Result<i32> {
        let size = self.env.call_method(self.inner.inner, "size", "()I", &[])?;
        size.i()
    }

    /// Returns an iterator over the elements in this set.
    pub fn iterator(&self) -> Result<Iterator<'a>> {
        let iterator = self.env.call_method(self.inner.inner, "iterator", "()Ljava/util/Iterator;", &[])?;
        let object = Object::new(self.env, iterator.l()?, Class::Iterator(self.env)?);
        let iterator = Iterator::new(self.env, object, self.class.clone());
        Ok(iterator)
    }

    /// Convert the java.util.Set to a Vec
    pub fn to_vec(&self) -> Result<Vec<Object<'a>>> {
        let mut vec = Vec::new();
        let iter = self.iterator()?;
        while let Some(i) = iter.next()? {
            vec.push(i);
        }

        Ok(vec)
    }
}