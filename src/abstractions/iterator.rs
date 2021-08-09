use jni::errors::Result;
use jni::JNIEnv;
use crate::object::Object;
use crate::class::Class;
use jni::sys::_jobject;

/// A wrapper for a java.util.Iterator
pub struct Iterator<'a> {
    /// The iterator itself
    pub inner: Object<'a>,

    /// The Class the iterator iterates over
    pub class: Class<'a>
}

impl<'a> Into<*mut _jobject> for Iterator<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.obj.into_inner()
    }
}

impl<'a> Iterator<'a> {
    /// Create a new instance of Iterator. The caller must guarantee that the passed in Object implements Iterator and is not null.
    pub fn new(object: Object<'a>, class: Class<'a>) -> Self {
        Self {
            inner: object,
            class
        }
    }

    /// Returns true if the iteration has more elements.
    pub fn has_next(&self, env: &JNIEnv<'a>) -> Result<bool> {
        let has_next = env.call_method(self.inner.obj, "hasNext", "()Z", &[])?;
        Ok(has_next.z()?)
    }

    /// Returns the next element in the iteration.
    pub fn next(&self, env: &JNIEnv<'a>) -> Result<Option<Object<'a>>> {
        if !self.has_next(env)? {
            return Ok(None)
        }

        let next = env.call_method(self.inner.obj, "next", "()Ljava/lang/Object;", &[])?;
        let object = Object::new(next.l()?, self.class.0.into());
        match object.obj.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }
}