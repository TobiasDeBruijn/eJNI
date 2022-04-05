use crate::class::Class;
use crate::object::Object;
use jni::objects::JValue;
use jni::sys::_jobject;
use jni::{errors::Result, JNIEnv};

/// Wrapper around `java.util.List`
pub struct List<'a> {
    /// The list itself
    pub inner: Object<'a>,

    /// The type contained in the List
    pub class: Class<'a>,

    env: &'a JNIEnv<'a>,
}

impl<'a> From<Object<'a>> for List<'a> {
    fn from(obj: Object<'a>) -> Self {
        Self {
            inner: obj.clone(),
            class: obj.class,
            env: obj.env,
        }
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<*mut _jobject> for List<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.inner.into_inner()
    }
}

impl<'a> Drop for List<'a> {
    fn drop(&mut self) {
        let _ = self.env.delete_local_ref(self.inner.inner);
    }
}

impl<'a> List<'a> {
    /// Create a List abstraction from it's raw components. The caller must guarantee that `object` implements `java.util.List` and that `class` is the correct Class
    pub fn new(env: &'a JNIEnv<'a>, object: Object<'a>, class: Class<'a>) -> Self {
        Self {
            inner: object,
            class,
            env,
        }
    }

    /// Create a new `java.util.ArrayList`
    pub fn arraylist(env: &'a JNIEnv<'a>, v_class: Class<'a>) -> Result<Self> {
        let arraylist = env.new_object("java/util/ArrayList", "()V", &[])?;
        Ok(Self {
            inner: Object::new(env, arraylist, Class::ArrayList(env)?),
            class: v_class,
            env,
        })
    }

    /// Appends the specified element to the end of this list (optional operation).
    pub fn add(&self, object: &Object<'a>) -> Result<bool> {
        let object = self.env.call_method(
            self.inner.inner,
            "add",
            "(Ljava/lang/Object;)Z",
            &[object.into()],
        )?;
        object.z()
    }

    /// Inserts the specified element at the specified position in this list (optional operation).
    pub fn add_at(&self, object: &Object<'a>, index: i32) -> Result<()> {
        self.env.call_method(
            self.inner.inner,
            "add",
            "(ILjava/lang/Object;)V",
            &[JValue::Int(index), object.into()],
        )?;
        Ok(())
    }

    /// Removes all of the elements from this list (optional operation).
    pub fn clear(&self) -> Result<()> {
        self.env
            .call_method(self.inner.inner, "clear", "()V", &[])?;
        Ok(())
    }

    /// Returns true if this list contains the specified element.
    pub fn contains(&self, object: &Object<'a>) -> Result<bool> {
        let contains = self.env.call_method(
            self.inner.inner,
            "contains",
            "(Ljava/lang/Object;)Z",
            &[object.into()],
        )?;
        contains.z()
    }

    /// Returns the element at the specified position in this list.
    pub fn get(&self, index: i32) -> Result<Option<Object<'a>>> {
        let value = self.env.call_method(
            self.inner.inner,
            "get",
            "(I)Ljava/lang/Object;",
            &[JValue::Int(index)],
        )?;
        let maybe_object = value.l()?;
        match maybe_object.is_null() {
            true => Ok(None),
            false => Ok(Some(Object::new(
                self.env,
                maybe_object,
                self.class.clone(),
            ))),
        }
    }

    /// Returns the index of the first occurrence of the specified element in this list, or -1 if this list does not contain the element.
    pub fn index_of(&self, object: &Object<'a>) -> Result<i32> {
        let index = self.env.call_method(
            self.inner.inner,
            "indexOf",
            "(Ljava/lang/Object;)I",
            &[object.into()],
        )?;
        index.i()
    }

    /// Returns true if this list contains no elements.
    pub fn is_empty(&self) -> Result<bool> {
        let is_empty = self
            .env
            .call_method(self.inner.inner, "isEmpty", "()Z", &[])?;
        is_empty.z()
    }

    /// Removes the first occurrence of the specified element from this list, if it is present (optional operation).
    pub fn remove(&self, object: &Object<'a>) -> Result<bool> {
        let remove = self.env.call_method(
            self.inner.inner,
            "remove",
            "(Ljava/lang/Object;)Z",
            &[object.into()],
        )?;
        remove.z()
    }

    /// Removes the element at the specified position in this list (optional operation).
    pub fn remove_at(&self, index: i32) -> Result<Option<Object<'a>>> {
        let value = self.env.call_method(
            self.inner.inner,
            "remove",
            "(I)Ljava/lang/Object;",
            &[JValue::Int(index)],
        )?;
        let maybe_removed = value.l()?;
        match maybe_removed.is_null() {
            true => Ok(None),
            false => Ok(Some(Object::new(
                self.env,
                maybe_removed,
                self.class.clone(),
            ))),
        }
    }

    /// Replaces the element at the specified position in this list with the specified element (optional operation).
    pub fn set(&self, object: &Object<'a>, index: i32) -> Result<Option<Object<'a>>> {
        let replaced = self.env.call_method(
            self.inner.inner,
            "set",
            "(ILjava/lang/Object;)Ljava/lang/Object;",
            &[JValue::Int(index), object.into()],
        )?;
        let maybe_replaced = replaced.l()?;
        match maybe_replaced.is_null() {
            true => Ok(None),
            false => Ok(Some(Object::new(
                self.env,
                maybe_replaced,
                self.class.clone(),
            ))),
        }
    }

    /// Returns the number of elements in this list.
    pub fn size(&self) -> Result<i32> {
        let size = self.env.call_method(self.inner.inner, "size", "()I", &[])?;
        size.i()
    }

    /// Returns a view of the portion of this list between the specified from index, inclusive, and to index, exclusive.
    pub fn sublist(&self, from: i32, to: i32) -> Result<List<'a>> {
        let sublist = self.env.call_method(
            self.inner.inner,
            "subList",
            "(II)Ljava/util/List;",
            &[JValue::Int(from), JValue::Int(to)],
        )?;
        Ok(Self::new(
            self.env,
            Object::new(self.env, sublist.l()?, self.inner.class.clone()),
            self.class.clone(),
        ))
    }

    /// Returns an iterator over the elements in this list in proper sequence.
    pub fn iterator(&self) -> Result<crate::Iterator<'a>> {
        let iterator =
            self.env
                .call_method(self.inner.inner, "iterator", "()Ljava/util/Iterator;", &[])?;
        Ok(crate::Iterator::new(
            self.env,
            Object::new(self.env, iterator.l()?, Class::Iterator(self.env)?),
            self.class.clone(),
        ))
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use crate::class::Class;
    use crate::object::Object;
    use crate::test::JVM;

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

        let has_changed = list
            .add(&Object::new_integer_object(&env, 10).unwrap())
            .unwrap();
        assert!(has_changed);

        let size = list.size().unwrap();
        assert_eq!(1, size);
    }

    #[test]
    fn add_at() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        list.add(&Object::new_integer_object(&env, 20).unwrap())
            .unwrap();

        list.add_at(&Object::new_integer_object(&env, 10).unwrap(), 0)
            .unwrap();

        let zeroth = list.get(0).unwrap().unwrap();
        let zeroth_int = env
            .call_method(zeroth.inner, "intValue", "()I", &[])
            .unwrap()
            .i()
            .unwrap();
        assert_eq!(10, zeroth_int);

        let first = list.get(1).unwrap().unwrap();
        let first_int = env
            .call_method(first.inner, "intValue", "()I", &[])
            .unwrap()
            .i()
            .unwrap();
        assert_eq!(20, first_int);
    }

    #[test]
    fn clear() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        list.add(&Object::new_integer_object(&env, 20).unwrap())
            .unwrap();

        list.clear().unwrap();

        let size = list.size().unwrap();
        assert_eq!(0, size);
    }

    #[test]
    fn contains() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_integer_object(&env, 20).unwrap();
        list.add(&integer).unwrap();

        let contains = list.contains(&integer).unwrap();
        assert!(contains)
    }

    #[test]
    fn get() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_integer_object(&env, 20).unwrap();
        list.add(&integer).unwrap();

        let zeroth = list.get(0).unwrap().unwrap();
        assert!(integer.equals(&zeroth).unwrap())
    }

    #[test]
    fn index_of() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_integer_object(&env, 20).unwrap();
        list.add(&integer).unwrap();

        let index_of = list.index_of(&integer).unwrap();
        assert_eq!(0, index_of);
    }

    #[test]
    fn is_empty() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();

        let is_empty = list.is_empty().unwrap();
        assert!(is_empty);
    }

    #[test]
    fn remove() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_integer_object(&env, 20).unwrap();
        list.add(&integer).unwrap();

        let list_changed = list.remove(&integer).unwrap();
        assert!(list_changed);
    }

    #[test]
    fn remove_at() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_integer_object(&env, 20).unwrap();
        list.add(&integer).unwrap();

        let removed = list.remove_at(0).unwrap().unwrap();
        let equals = integer.equals(&removed).unwrap();

        assert!(equals)
    }

    #[test]
    fn set() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_integer_object(&env, 20).unwrap();
        list.add(&integer).unwrap();

        let new_integer = Object::new_integer_object(&env, 10).unwrap();

        let old_integer = list.set(&new_integer, 0).unwrap().unwrap();
        let equal = integer.equals(&old_integer).unwrap();
        assert!(equal);
    }

    #[test]
    fn size() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        let integer = Object::new_integer_object(&env, 20).unwrap();
        list.add(&integer).unwrap();

        let size = list.size().unwrap();
        assert_eq!(1, size);
    }

    #[test]
    fn sublist() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();
        list.add(&Object::new_integer_object(&env, 10).unwrap())
            .unwrap();
        list.add(&Object::new_integer_object(&env, 20).unwrap())
            .unwrap();
        list.add(&Object::new_integer_object(&env, 30).unwrap())
            .unwrap();

        let sublist = list.sublist(1, 3).unwrap();
        let size = sublist.size().unwrap();
        assert_eq!(2, size);
    }

    #[test]
    fn iterator() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();

        let iterator = list.iterator();
        assert!(iterator.is_ok());
    }
}
