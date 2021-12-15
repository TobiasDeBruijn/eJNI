use crate::object::Object;
use jni::errors::Result;
use jni::JNIEnv;
use jni::objects::JValue;
use crate::class::Class;
use crate::abstractions::set::Set;
use jni::sys::_jobject;

/// Wrapper around `java.util.Map`
pub struct Map<'a> {
    /// The Map itself
    pub inner:      Object<'a>,

    /// The key Class
    pub k_class:    Class<'a>,

    /// The Value Class
    pub v_class:    Class<'a>,

    env:            &'a JNIEnv<'a>
}

#[allow(clippy::from_over_into)]
impl<'a> Into<*mut _jobject> for Map<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.inner.into_inner()
    }
}

impl<'a> Drop for Map<'a> {
    fn drop(&mut self) {
        let _ = self.env.delete_local_ref(self.inner.inner);
    }
}

impl<'a> Map<'a> {
    /// Create a Map wrapper from an existing Map object. The caller must guarantee that the passed in Object implements Map and is not null.
    pub fn new(env: &'a JNIEnv<'a>, object: Object<'a>, k_class: Class<'a>, v_class: Class<'a>) -> Self {
        Self {
            inner: object,
            k_class,
            v_class,
            env
        }
    }

    /// Constructs an empty HashMap with the default initial capacity (16) and the default load factor (0.75).
    pub fn hashmap(env: &'a JNIEnv<'a>, k_class: Class<'a>, v_class: Class<'a>) -> Result<Self> {
        let hashmap = env.new_object("java/util/HashMap", "()V", &[])?;
        Ok(Self {
            inner: Object::new(env, hashmap, Class::HashMap(env)?),
            k_class,
            v_class,
            env
        })
    }

    /// Constructs an empty HashMap with the specified initial capacity and the default load factor (0.75).
    pub fn hashmap_with_capacity(env: &'a JNIEnv<'a>, k_class: Class<'a>, v_class: Class<'a>, initial_capacity: i32) -> Result<Self> {
        let hashmap = env.new_object("java/util/HashMap", "(I)V", &[JValue::Int(initial_capacity)])?;
        Ok(Self {
            inner: Object::new(env, hashmap, Class::HashMap(env)?),
            k_class,
            v_class,
            env
        })
    }

    /// Constructs an empty HashMap with the specified initial capacity and load factor.
    pub fn hashmap_with_capacity_and_load_factor(env: &'a JNIEnv<'a>, k_class: Class<'a>, v_class: Class<'a>, initial_capacity: i32, load_factor: f32) -> Result<Self> {
        let hashmap = env.new_object("java/util/HashMap", "(IF)V", &[JValue::Int(initial_capacity), JValue::Float(load_factor)])?;
        Ok(Self {
            inner: Object::new(env, hashmap, Class::HashMap(env)?),
            k_class,
            v_class,
            env
        })
    }

    /// Associates the specified value with the specified key in this map (optional operation).
    pub fn put(&self, key: Object<'a>, value: Object<'a>) -> Result<Option<Object<'a>>> {
        let prev_value = self.env.call_method(self.inner.inner, "put", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;", &[key.into(), value.into()])?;
        let object = Object::new(self.env, prev_value.l()?, self.v_class.clone());
        match object.inner.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }

    /// Returns the value to which the specified key is mapped, or null if this map contains no mapping for the key.
    pub fn get(&self, key: &Object<'a>) -> Result<Option<Object<'a>>> {
        let value = self.env.call_method(self.inner.inner, "get", "(Ljava/lang/Object;)Ljava/lang/Object;", &[key.into()])?;
        let object = Object::new(self.env, value.l()?, self.v_class.clone());
        match object.inner.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }

    /// Returns true if this map contains no key-value mappings.
    pub fn is_empty(&self) -> Result<bool> {
        let is_empty = self.env.call_method(self.inner.inner, "isEmpty", "()Z", &[])?;
        is_empty.z()
    }

    /// Returns the number of key-value mappings in this map.
    pub fn size(&self) -> Result<i32> {
        let size = self.env.call_method(self.inner.inner, "size", "()I", &[])?;
        size.i()
    }

    /// Returns true if this map contains a mapping for the specified key.
    pub fn contains_key(&self, key: &Object<'a>) -> Result<bool> {
        let contains_key = self.env.call_method(self.inner.inner, "containsKey", "(Ljava/lang/Object;)Z", &[key.into()])?;
        contains_key.z()
    }

    /// Returns true if this map maps one or more keys to the specified value.
    pub fn contains_value(&self, value: &Object<'a>) -> Result<bool> {
        let contains_value = self.env.call_method(self.inner.inner, "containsValue", "(Ljava/lang/Object;)Z", &[value.into()])?;
        contains_value.z()
    }

    /// Removes the mapping for a key from this map if it is present (optional operation).
    pub fn remove(&self, key: &Object<'a>) -> Result<Option<Object<'a>>> {
        let removed_value = self.env.call_method(self.inner.inner, "remove", "(Ljava/lang/Object;)Ljava/lang/Object;", &[key.into()])?;
        let object = Object::new(self.env, removed_value.l()?, self.v_class.clone());
        match object.inner.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }

    /// Removes the entry for the specified key only if it is currently mapped to the specified value.
    pub fn remove_if_mapped(&self, key: &Object<'a>, value: &Object<'a>) -> Result<bool> {
        let removed = self.env.call_method(self.inner.inner, "remove", "(Ljava/lang/Object;Ljava/lang/Object;)Z", &[key.into(), value.into()])?;
        removed.z()
    }

    /// Returns a Set<Map.Entry<K, V>> view of the mappings contained in this map.
    pub fn entry_set(&self) -> Result<Set<'a>> {
        let entry_set = self.env.call_method(self.inner.inner, "entrySet", "()Ljava/util/Set;", &[])?;
        let object = Object::new(self.env, entry_set.l()?, Class::Set(self.env)?);
        let set = Set::new(self.env, object, Class::MapEntry(self.env)?);
        Ok(set)
    }

    /// Removes all of the mappings from this map.
    pub fn clear(&self) -> Result<()> {
        self.env.call_method(self.inner.inner, "clear", "()V", &[])?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::test::JVM;
    use super::*;

    #[test]
    fn hashmap() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();

        assert!(Map::hashmap(&env, int_class.clone(), int_class.clone()).is_ok());
        assert!(Map::hashmap_with_capacity(&env, int_class.clone(), int_class.clone(), 32).is_ok());
        assert!(Map::hashmap_with_capacity_and_load_factor(&env, int_class.clone(), int_class, 32, 32.5).is_ok());
    }

    #[test]
    fn put() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        let put_result = map.put(Object::new_integer_object(&env, 1).unwrap(), Object::new_integer_object(&env, 10).unwrap());
        assert!(put_result.is_ok());
    }

    #[test]
    fn get() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        let key = Object::new_integer_object(&env, 1).unwrap();

        map.put(key.clone(), Object::new_integer_object(&env, 10).unwrap()).unwrap();
        let gotten = map.get(&key).unwrap();
        assert!(gotten.is_some());

        let gotten = gotten.unwrap().get_integer().unwrap();
        assert_eq!(10, gotten);
    }

    #[test]
    fn is_empty() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        let is_empty = map.is_empty();
        assert!(is_empty.is_ok());
        let is_empty = is_empty.unwrap();
        assert!(is_empty);
    }

    #[test]
    fn size() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        map.put(
            Object::new_integer_object(&env, 1).unwrap(),
            Object::new_integer_object(&env, 10).unwrap()
        ).unwrap();

        let size = map.size();
        assert!(size.is_ok());

        let size = size.unwrap();
        assert_eq!(1, size);
    }

    #[test]
    fn contains_key() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        let key = Object::new_integer_object(&env, 1).unwrap();

        map.put(
            key.clone(),
            Object::new_integer_object(&env, 10).unwrap()
        ).unwrap();

        let contains_key = map.contains_key(&key);
        assert!(contains_key.is_ok());

        let contains_key = contains_key.unwrap();
        assert!(contains_key);
    }

    #[test]
    fn contains_value() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        let value =  Object::new_integer_object(&env, 10).unwrap();

        map.put(
            Object::new_integer_object(&env, 1).unwrap(),
            value.clone()
        ).unwrap();

        let contains_value = map.contains_value(&value);
        assert!(contains_value.is_ok());

        let contains_value = contains_value.unwrap();
        assert!(contains_value);
    }

    #[test]
    fn remove() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        let key = Object::new_integer_object(&env, 1).unwrap();
        let value = Object::new_integer_object(&env, 10).unwrap();

        map.put(
            key.clone(),
            value.clone()
        ).unwrap();

        let removed = map.remove(&key);
        assert!(removed.is_ok());

        let removed = removed.unwrap();
        assert!(removed.is_some());

        let removed = removed.unwrap();
        assert!(value.equals(&removed).unwrap())
    }

    #[test]
    fn entry_set() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        let entry_set = map.entry_set();
        assert!(entry_set.is_ok());
    }

    #[test]
    fn remove_if_mapped() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        let key = Object::new_integer_object(&env, 1).unwrap();
        let value = Object::new_integer_object(&env, 10).unwrap();

        map.put(key.clone(), value.clone()).unwrap();
        assert_eq!(1, map.size().unwrap());

        let other_value = Object::new_integer_object(&env, 25).unwrap();
        assert!(map.remove_if_mapped(&key, &other_value).is_ok());
        assert_eq!(1, map.size().unwrap());

        assert!(map.remove_if_mapped(&key, &value).unwrap());
        assert_eq!(0, map.size().unwrap());
    }

    #[test]
    fn clear() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let map = Map::hashmap(&env, int_class.clone(), int_class).unwrap();

        map.put(
            Object::new_integer_object(&env, 1).unwrap(),
            Object::new_integer_object(&env, 10).unwrap()
        ).unwrap();

        let old_size = map.size().unwrap();
        assert_eq!(1, old_size);

        assert!(map.clear().is_ok());

        let new_size = map.size().unwrap();
        assert_eq!(0, new_size);
    }
}