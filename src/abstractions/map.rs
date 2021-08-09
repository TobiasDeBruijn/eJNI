use crate::object::Object;
use jni::errors::Result;
use jni::JNIEnv;
use crate::class::Class;
use crate::abstractions::set::Set;
use jni::sys::_jobject;

/// Wrapper around java.util.Map
pub struct Map<'a> {
    /// The Map itself
    pub inner: Object<'a>,

    /// The key Class
    pub k_class: Class<'a>,

    /// The Value Class
    pub v_class: Class<'a>
}

impl<'a> Into<*mut _jobject> for Map<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.obj.into_inner()
    }
}

impl<'a> Map<'a> {
    #![allow(non_snake_case)]

    /// Create a Map wrapper from an existing Map object. The caller must guarantee that the passed in Object implements Map and is not null.
    pub fn new(object: Object<'a>, k_class: Class<'a>, v_class: Class<'a>) -> Self {
        Self {
            inner: object,
            k_class,
            v_class
        }
    }

    /// Create a Map wrapper for a new HashMap
    pub fn hashmap(env: &JNIEnv<'a>, k_class: Class<'a>, v_class: Class<'a>) -> Result<Self> {
        let hashmap = env.new_object("java/util/HashMap", "()V", &[])?;

        Ok(Self {
            inner: Object::new(hashmap, Class::HashMap(env)?),
            k_class,
            v_class
        })
    }

    /// Associates the specified value with the specified key in this map (optional operation).
    pub fn put(&self, env: &JNIEnv<'a>, key: Object<'a>, value: Object<'a>) -> Result<Option<Object<'a>>> {
        let prev_value = env.call_method(self.inner.obj, "put", "(Ljava/lang/Object;Ljava/lang/Object)Ljava/lang/Object;", &[key.into(), value.into()])?;
        let object = Object::new(prev_value.l()?, self.v_class.0.into());
        match object.obj.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }

    /// Returns the value to which the specified key is mapped, or null if this map contains no mapping for the key.
    pub fn get(&self, env: &JNIEnv<'a>, key: &Object<'a>) -> Result<Option<Object<'a>>> {
        let value = env.call_method(self.inner.obj, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", &[key.into()])?;
        let object = Object::new(value.l()?, self.v_class.0.into());
        match object.obj.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }

    /// Returns true if this map contains no key-value mappings.
    pub fn is_empty(&self, env: &JNIEnv<'a>) -> Result<bool> {
        let is_empty = env.call_method(self.inner.obj, "isEmpty", "()Z", &[])?;
        Ok(is_empty.z()?)
    }

    /// Returns the number of key-value mappings in this map.
    pub fn size(&self, env: &JNIEnv<'a>) -> Result<i32> {
        let size = env.call_method(self.inner.obj, "size", "()I", &[])?;
        Ok(size.i()?)
    }

    /// Returns true if this map contains a mapping for the specified key.
    pub fn contains_key(&self, env: &JNIEnv<'a>, key: &Object<'a>) -> Result<bool> {
        let contains_key = env.call_method(self.inner.obj, "containsKey", "(Ljava/lang/Object;)Z", &[key.into()])?;
        Ok(contains_key.z()?)
    }

    /// Returns true if this map maps one or more keys to the specified value.
    pub fn contains_value(&self, env: &JNIEnv<'a>, value: &Object<'a>) -> Result<bool> {
        let contains_value = env.call_method(self.inner.obj, "containsValue", "(Ljava/lang/Object;)Z", &[value.into()])?;
        Ok(contains_value.z()?)
    }

    /// Removes the mapping for a key from this map if it is present (optional operation).
    pub fn remove(&self, env: &JNIEnv<'a>, key: &Object<'a>) -> Result<Option<Object<'a>>> {
        let removed_value = env.call_method(self.inner.obj, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", &[key.into()])?;
        let object = Object::new(removed_value.l()?, self.v_class.0.into());
        match object.obj.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }

    /// Removes the entry for the specified key only if it is currently mapped to the specified value.
    pub fn remove_if_mapped(&self, env: &JNIEnv<'a>, key: &Object<'a>, value: &Object<'a>) -> Result<bool> {
        let removed = env.call_method(self.inner.obj, "remove", "(Ljava/lang/Object;Ljava/lang/Object;)Z", &[key.into(), value.into()])?;
        Ok(removed.z()?)
    }

    /// Returns a Set<Map.Entry<K, V>> view of the mappings contained in this map.
    pub fn entry_set(&self, env: &JNIEnv<'a>) -> Result<Set<'a>> {
        let entry_set = env.call_method(self.inner.obj, "entrySet()", "()Ljava/util/Set;", &[])?;
        let object = Object::new(entry_set.l()?, Class::Set(env)?);
        let set = Set::new(object, Class::MapEntry(env)?);
        Ok(set)
    }
}