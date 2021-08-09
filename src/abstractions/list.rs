use crate::object::Object;
use crate::class::Class;
use jni::{JNIEnv, errors::Result};
use jni::objects::JValue;

/// A wrapper around `java.util.List`
pub struct List<'a> {
    /// The list itself
    pub inner: Object<'a>,

    /// The type contained in the List
    pub class: Class<'a>
}

impl<'a> List<'a> {
    pub fn new(object: Object<'a>, class: Class<'a>) -> Self {
        Self {
            inner: object,
            class
        }
    }

    /// Create a new `java.util.ArrayList`
    pub fn arraylist(env: &JNIEnv<'a>, v_class: Class<'a>) -> Result<Self> {
        let arraylist = env.new_object("java/util/ArrayList", "()V", &[])?;
        Ok(Self {
            inner: Object::new(arraylist, Class::ArrayList(env)?),
            class: v_class
        })
    }

    /// Appends the specified element to the end of this list (optional operation).
    pub fn add(&self, env: &JNIEnv<'a>, object: &Object<'a>) -> Result<bool> {
        let object = env.call_method(self.inner.obj, "add", "(Ljava/lang/Object;)Z", &[object.into()])?;
        Ok(object.z()?)
    }

    /// Inserts the specified element at the specified position in this list (optional operation).
    pub fn add_at(&self, env: &JNIEnv<'a>, object: &Object<'a>, index: i32) -> Result<()> {
        env.call_method(self.inner.obj, "add", "(ILjava/lang/Object;)V", &[JValue::Int(index), object.into()])?;
        Ok(())
    }

    /// Removes all of the elements from this list (optional operation).
    pub fn clear(&self, env: &JNIEnv<'a>) -> Result<()> {
        env.call_method(self.inner.obj, "clear", "()V", &[])?;
        Ok(())
    }

    /// Returns true if this list contains the specified element.
    pub fn contains(&self, env: &JNIEnv<'a>, object: &Object<'a>) -> Result<bool> {
        let contains = env.call_method(self.inner.obj, "contains", "(Ljava/lang/Object;)Z", &[object.into()])?;
        Ok(contains.z()?)
    }

    /// Returns the element at the specified position in this list.
    pub fn get(&self, env: &JNIEnv<'a>, index: i32) -> Result<Option<Object<'a>>> {
        let value = env.call_method(self.inner.obj, "get", "(I)Ljava/lang/Object;", &[JValue::Int(index)])?;
        let maybe_object = value.l()?;
        match maybe_object.is_null() {
            true => Ok(None),
            false => Ok(Some(Object::new(maybe_object, self.class.0.into())))
        }
    }

    /// Returns the index of the first occurrence of the specified element in this list, or -1 if this list does not contain the element.
    pub fn index_of(&self, env: &JNIEnv<'a>, object: &Object<'a>) -> Result<i32> {
        let index = env.call_method(self.inner.obj, "indexOf", "(Ljava/lang/Object;)I", &[object.into()])?;
        Ok(index.i()?)
    }

    /// Returns true if this list contains no elements.
    pub fn is_empty(&self, env: &JNIEnv<'a>) -> Result<bool> {
        let is_empty = env.call_method(self.inner.obj, "isEmpty", "()Z", &[])?;
        Ok(is_empty.z()?)
    }

    /// Removes the first occurrence of the specified element from this list, if it is present (optional operation).
    pub fn remove(&self, env: &JNIEnv<'a>, object: &Object<'a>) -> Result<bool> {
        let remove = env.call_method(self.inner.obj, "remove", "(Ljava/lang/Object;)Z", &[object.into()])?;
        Ok(remove.z()?)
    }

    /// Removes the element at the specified position in this list (optional operation).
    pub fn remove_at(&self, env: &JNIEnv<'a>, index: i32) -> Result<Option<Object<'a>>> {
        let value = env.call_method(self.inner.obj, "remove", "(I)Ljava/lang/Object;", &[JValue::Int(index)])?;
        let maybe_removed = value.l()?;
        match maybe_removed.is_null() {
            true => Ok(None),
            false => Ok(Some(Object::new(maybe_removed, self.class.0.into())))
        }
    }

    /// Replaces the element at the specified position in this list with the specified element (optional operation).
    pub fn set(&self, env: &JNIEnv<'a>, object: &Object<'a>, index: i32) -> Result<Option<Object<'a>>> {
        let replaced = env.call_method(self.inner.obj, "set", "(ILjava/lang/Object;)Ljava/lang/Object;", &[JValue::Int(index), object.into()])?;
        let maybe_replaced = replaced.l()?;
        match maybe_replaced.is_null() {
            true => Ok(None),
            false => Ok(Some(Object::new(maybe_replaced, self.class.0.into())))
        }
    }

    /// Returns the number of elements in this list.
    pub fn size(&self, env: &JNIEnv<'a>) -> Result<i32> {
        let size = env.call_method(self.inner.obj, "size", "()I", &[])?;
        Ok(size.i()?)
    }

    /// Returns a view of the portion of this list between the specified from index, inclusive, and to index, exclusive.
    pub fn sublist(&self, env: &JNIEnv<'a>, from: i32, to: i32) -> Result<List<'a>> {
        let sublist = env.call_method(self.inner.obj, "subList", "(II)Ljava/util/List;", &[JValue::Int(from), JValue::Int(to)])?;
        Ok(Self::new(Object::new(sublist.l()?, self.inner.class.0.into()), self.class.0.into()))
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use crate::class::Class;
    use crate::test::JVM;
    use crate::object::Object;

    #[test]
    fn arraylist() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        assert!(List::arraylist(&env, Class::Object(&env).unwrap()).is_ok())
    }

    #[test]
    fn add() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();

        let has_changed = list.add(&env, &Object::new_Integer(&env, 10).unwrap()).unwrap();
        assert!(has_changed);

        let size = list.size(&env).unwrap();
        assert_eq!(1, size);
    }

    #[test]
    fn add_at() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        list.add(&env, &Object::new_Integer(&env, 20).unwrap()).unwrap();

        list.add_at(&env, &Object::new_Integer(&env, 10).unwrap(), 0).unwrap();

        let zeroth = list.get(&env, 0).unwrap().unwrap();
        let zeroth_int = env.call_method(zeroth.obj, "intValue", "()I", &[]).unwrap().i().unwrap();
        assert_eq!(10, zeroth_int);

        let first = list.get(&env, 1).unwrap().unwrap();
        let first_int = env.call_method(first.obj, "intValue", "()I", &[]).unwrap().i().unwrap();
        assert_eq!(20, first_int);
    }

    #[test]
    fn clear() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        list.add(&env, &Object::new_Integer(&env, 20).unwrap()).unwrap();

        list.clear(&env).unwrap();

        let size = list.size(&env).unwrap();
        assert_eq!(0, size);
    }

    #[test]
    fn contains() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_Integer(&env, 20).unwrap();
        list.add(&env, &integer).unwrap();

        let contains = list.contains(&env, &integer).unwrap();
        assert!(contains)
    }

    #[test]
    fn get() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_Integer(&env, 20).unwrap();
        list.add(&env, &integer).unwrap();

        let zeroth = list.get(&env, 0).unwrap().unwrap();
        assert!(integer.equals(&env, &zeroth).unwrap())
    }

    #[test]
    fn index_of() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_Integer(&env, 20).unwrap();
        list.add(&env, &integer).unwrap();

        let index_of = list.index_of(&env, &integer).unwrap();
        assert_eq!(0, index_of);
    }

    #[test]
    fn is_empty() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();

        let is_empty = list.is_empty(&env).unwrap();
        assert!(is_empty);
    }

    #[test]
    fn remove() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_Integer(&env, 20).unwrap();
        list.add(&env, &integer).unwrap();

        let list_changed = list.remove(&env, &integer).unwrap();
        assert!(list_changed);
    }

    #[test]
    fn remove_at() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_Integer(&env, 20).unwrap();
        list.add(&env, &integer).unwrap();

        let removed = list.remove_at(&env, 0).unwrap().unwrap();
        let equals = integer.equals(&env, &removed).unwrap();

        assert!(equals)
    }

    #[test]
    fn set() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_Integer(&env, 20).unwrap();
        list.add(&env, &integer).unwrap();

        let new_integer = Object::new_Integer(&env, 10).unwrap();

        let old_integer = list.set(&env, &new_integer, 0).unwrap().unwrap();
        let equal = integer.equals(&env, &old_integer).unwrap();
        assert!(equal);
    }

    #[test]
    fn size() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_Integer(&env, 20).unwrap();
        list.add(&env, &integer).unwrap();

        let size = list.size(&env).unwrap();
        assert_eq!(1, size);
    }

    #[test]
    fn sublist() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        list.add(&env, &Object::new_Integer(&env, 10).unwrap()).unwrap();
        list.add(&env, &Object::new_Integer(&env, 20).unwrap()).unwrap();
        list.add(&env, &Object::new_Integer(&env, 30).unwrap()).unwrap();

        let sublist = list.sublist(&env, 1, 3).unwrap();
        let size = sublist.size(&env).unwrap();
        assert_eq!(2, size);
    }
}