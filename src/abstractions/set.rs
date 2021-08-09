use crate::object::Object;
use crate::class::Class;
use jni::JNIEnv;
use jni::errors::Result;
use crate::abstractions::iterator::Iterator;
use jni::sys::_jobject;

/// Wrapper around java.util.Set
pub struct Set<'a> {
    /// The Set itself
    pub inner: Object<'a>,

    /// The Class contained in the Set
    pub class: Class<'a>
}

impl<'a> Into<*mut _jobject> for Set<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.obj.into_inner()
    }
}

impl<'a> Set<'a> {
    /// Create a new Set. The caller must guarantee that the passed in Object implements Set and is not null.
    pub fn new(object: Object<'a>, class: Class<'a>) -> Self {
        Self {
            inner: object,
            class
        }
    }

    /// Returns the number of elements in this set (its cardinality).
    pub fn size(&self, env: &JNIEnv<'a>) -> Result<i32> {
        let size = env.call_method(self.inner.obj, "size", "()I", &[])?;
        Ok(size.i()?)
    }

    /// Returns an iterator over the elements in this set.
    pub fn iterator(&self, env: &JNIEnv<'a>) -> Result<Iterator<'a>> {
        let iterator = env.call_method(self.inner.obj, "iterator", "()Ljava/util/Iterator;", &[])?;
        let object = Object::new(iterator.l()?, Class::Iterator(env)?);
        let iterator = Iterator::new(object, self.class.0.into());
        Ok(iterator)
    }

    pub fn to_vec(&self, env: &JNIEnv<'a>) -> Result<Vec<Object<'a>>> {
        let mut vec = Vec::new();

        let iter = self.iterator(env)?;
        while iter.has_next(env)? {
            match iter.next(env)? {
                Some(next) => vec.push(next),
                None => continue
            }
        }

        Ok(vec)
    }
}