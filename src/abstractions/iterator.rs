use jni::errors::Result;
use jni::JNIEnv;
use crate::object::Object;
use crate::class::Class;
use jni::sys::_jobject;

/// Wrapper around `java.util.Iterator`
pub struct Iterator<'a> {
    /// The iterator itself
    pub inner:  Object<'a>,

    /// The Class the iterator iterates over
    pub class:  Class<'a>,

    env:        &'a JNIEnv<'a>
}

#[allow(clippy::from_over_into)]
impl<'a> Into<*mut _jobject> for Iterator<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.inner.into_inner()
    }
}

impl<'a> Drop for Iterator<'a> {
    fn drop(&mut self) {
        let _ = self.env.delete_local_ref(self.inner.inner);
    }
}

impl<'a> std::iter::Iterator for Iterator<'a> {
    type Item = Result<Object<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        Iterator::next(self).transpose()
    }
}

impl<'a> Iterator<'a> {
    /// Create a new instance of Iterator. The caller must guarantee that the passed in Object implements Iterator and is not null.
    pub fn new(env: &'a JNIEnv<'a>, object: Object<'a>, class: Class<'a>) -> Self {
        Self {
            inner: object,
            class,
            env
        }
    }

    /// Returns true if the iteration has more elements.
    pub fn has_next(&self) -> Result<bool> {
        let has_next = self.env.call_method(self.inner.inner, "hasNext", "()Z", &[])?;
        has_next.z()
    }

    /// Returns the next element in the iteration.
    pub fn next(&self) -> Result<Option<Object<'a>>> {
        if !self.has_next()? {
            return Ok(None)
        }

        let next = self.env.call_method(self.inner.inner, "next", "()Ljava/lang/Object;", &[])?;
        let object = Object::new(self.env,next.l()?, self.class.clone());
        match object.inner.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }

    /// Convert the java.util.Iterator to a Vec
    pub fn to_vec(&self) -> Result<Vec<Object>> {
        let mut objects = Vec::new();
        while let Some(i) = self.next()? {
            objects.push(i);
        }

        Ok(objects)
    }
}

#[cfg(test)]
mod test {
    use crate::List;
    use crate::test::JVM;
    use super::*;

    #[test]
    fn has_next() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let int_class = Class::Integer(&env).unwrap();
        let list = List::arraylist(&env, int_class).unwrap();

        let iterator = list.iterator().unwrap();
        let has_next = iterator.has_next();
        assert!(has_next.is_ok());
        assert!(!has_next.unwrap());

        list.add(&Object::new_integer_object(&env, 10).unwrap()).unwrap();
        let iterator = list.iterator().unwrap();
        let has_next = iterator.has_next();
        assert!(has_next.is_ok());
        assert!(has_next.unwrap());
    }

    #[test]
    fn next() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();

        let iterator = list.iterator().unwrap();
        let next = iterator.next();
        assert!(next.is_ok());
        assert!(next.unwrap().is_none());

        let value = Object::new_integer_object(&env, 10).unwrap();
        list.add(&value).unwrap();

        let iterator = list.iterator().unwrap();
        let next = iterator.next();
        assert!(next.is_ok());

        let next = next.unwrap();
        assert!(next.is_some());

        let next = next.unwrap();
        assert_eq!(value.get_integer().unwrap(), next.get_integer().unwrap());
    }

    #[test]
    fn to_vec() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let list = List::arraylist(&env, Class::Integer(&env).unwrap()).unwrap();

        let iterator = list.iterator().unwrap();
        let vec = iterator.to_vec();
        assert!(vec.is_ok());

        let vec = vec.unwrap();
        assert!(vec.is_empty());

        let value = Object::new_integer_object(&env, 10).unwrap();
        list.add(&value).unwrap();
        let iterator = list.iterator().unwrap();

        let vec = iterator.to_vec();
        assert!(vec.is_ok());
        let vec = vec.unwrap();

        assert!(!vec.is_empty());
        let front = vec.first();
        assert!(front.is_some());

        let front = front.unwrap();
        assert_eq!(value.get_integer().unwrap(), front.get_integer().unwrap());
    }
}