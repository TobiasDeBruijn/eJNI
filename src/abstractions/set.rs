use crate::object::Object;
use crate::class::Class;
use jni::JNIEnv;
use jni::errors::Result;
use crate::abstractions::iterator::Iterator;
use jni::sys::_jobject;

/// Wrapper around java.util.Set
pub struct Set<'a> {
    /// The Set itself
    pub inner:  Object<'a>,

    /// The Class contained in the Set
    pub class:  Class<'a>,

    env:        &'a JNIEnv<'a>
}

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

    /// Returns the number of elements in this set (its cardinality).
    pub fn size(&self) -> Result<i32> {
        let size = self.env.call_method(self.inner.inner, "size", "()I", &[])?;
        Ok(size.i()?)
    }

    /// Returns an iterator over the elements in this set.
    pub fn iterator(&self) -> Result<Iterator<'a>> {
        let iterator = self.env.call_method(self.inner.inner, "iterator", "()Ljava/util/Iterator;", &[])?;
        let object = Object::new(self.env, iterator.l()?, Class::Iterator(self.env)?);
        let iterator = Iterator::new(self.env, object, self.class.clone());
        Ok(iterator)
    }

    pub fn to_vec(&self) -> Result<Vec<Object<'a>>> {
        let mut vec = Vec::new();

        let iter = self.iterator()?;
        while iter.has_next()? {
            match iter.next()? {
                Some(next) => vec.push(next),
                None => continue
            }
        }

        Ok(vec)
    }
}