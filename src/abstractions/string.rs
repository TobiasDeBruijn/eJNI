use crate::object::Object;
use crate::class::Class;
use jni::JNIEnv;
use jni::errors::Result;
use jni::strings::JNIString;
use jni::objects::{JString, JValue};
use jni::sys::_jobject;

/// A wrapper for a java.lang.String
pub struct JavaString<'a>(pub Object<'a>);

impl<'a> Into<JValue<'a>> for JavaString<'a> {
    fn into(self) -> JValue<'a> {
        self.0.into()
    }
}

impl<'a> Into<*mut _jobject> for JavaString<'a> {
    fn into(self) -> *mut _jobject {
        self.0.obj.into_inner()
    }
}

impl<'a> JavaString<'a> {
    /// Create a JavaString wrapper. The caller must guarantee that the passed in Object is a java.lang.String and is not null.
    pub fn new(object: Object<'a>) -> Self {
        Self(object)
    }

    /// Turn a Rust String into a JavaString
    pub fn from_rust<S: Into<JNIString>>(env: &JNIEnv<'a>, s: S) -> Result<Self> {
        let string = env.new_string(s)?;
        Ok(Self::new(Object::new(string.into(), Class::String(env)?)))
    }

    /// Turn a JavaString into a Rust String
    pub fn into_rust(&self, env: &JNIEnv<'a>) -> Result<String> {
        Ok(env.get_string(JString::from(self.0.obj))?.into())
    }
}