use crate::object::Object;
use crate::class::Class;
use jni::JNIEnv;
use jni::errors::Result;
use jni::sys::_jobject;

/// Wrapper around Map.Entry
pub struct MapEntry<'a> {
    /// The Map.Entry itself
    pub inner: Object<'a>,

    /// The key Class
    pub k_class: Class<'a>,

    /// The value Class
    pub v_class: Class<'a>
}

impl<'a> Into<*mut _jobject> for MapEntry<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.obj.into_inner()
    }
}

impl<'a> MapEntry<'a> {
    /// Create a new Map.Entry wrapper. The caller must guarantee that the object passed in implements Map.Entry and that it is not null
    pub fn new(object: Object<'a>, k_class: Class<'a>, v_class: Class<'a>) -> Self {
        Self {
            inner: object,
            k_class,
            v_class
        }
    }

    /// Returns the key corresponding to this entry.
    pub fn get_key(&self, env: &JNIEnv<'a>) -> Result<Option<Object<'a>>> {
        let key = env.call_method(self.inner.obj, "getKey", "()Ljava/lang/Object;", &[])?;
        let object = Object::new(key.l()?, self.k_class.0.into());
        match object.obj.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }

    /// Returns the value corresponding to this entry.
    pub fn get_value(&self, env: &JNIEnv<'a>) -> Result<Option<Object<'a>>> {
        let value = env.call_method(self.inner.obj, "getValue", "()Ljava/lang/Object;", &[])?;
        let object = Object::new(value.l()?, self.v_class.0.into());
        match object.obj.is_null() {
            true => Ok(None),
            false => Ok(Some(object))
        }
    }
}