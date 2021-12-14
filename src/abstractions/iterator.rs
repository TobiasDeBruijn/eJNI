use jni::errors::Result;
use jni::JNIEnv;
use crate::object::Object;
use crate::class::Class;
use jni::sys::_jobject;

/// A wrapper for a java.util.Iterator
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
}