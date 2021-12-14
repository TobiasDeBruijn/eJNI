use crate::object::Object;
use crate::class::Class;
use jni::JNIEnv;
use jni::errors::Result;
use jni::sys::_jobject;

/// Wrapper around Map.Entry
pub struct MapEntry<'a> {
    /// The Map.Entry itself
    pub inner:      Object<'a>,

    /// The key Class
    pub k_class:    Class<'a>,

    /// The value Class
    pub v_class:    Class<'a>,

    env:            &'a JNIEnv<'a>
}

#[allow(clippy::from_over_into)]
impl<'a> Into<*mut _jobject> for MapEntry<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.inner.into_inner()
    }
}

impl<'a> Drop for MapEntry<'a> {
    fn drop(&mut self) {
        let _ = self.env.delete_local_ref(self.inner.inner);
    }
}

impl<'a> MapEntry<'a> {
    /// Create a new Map.Entry wrapper. The caller must guarantee that the object passed in implements Map.Entry and that it is not null
    pub fn new(env: &'a JNIEnv<'a>, object: Object<'a>, k_class: Class<'a>, v_class: Class<'a>) -> Self {
        Self {
            inner: object,
            k_class,
            v_class,
            env
        }
    }

    /// Returns the key corresponding to this entry.
    pub fn get_key(&self) -> Result<Option<Object<'a>>> {
        let key = self.env.call_method(self.inner.inner, "getKey", "()Ljava/lang/Object;", &[])?;
        let object = Object::new(self.env, key.l()?, self.k_class.clone());
        match object.inner.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }

    /// Returns the value corresponding to this entry.
    pub fn get_value(&self) -> Result<Option<Object<'a>>> {
        let value = self.env.call_method(self.inner.inner, "getValue", "()Ljava/lang/Object;", &[])?;
        let object = Object::new(self.env,value.l()?, self.v_class.clone());
        match object.inner.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }
}