use crate::object::Object;
use crate::class::Class;
use jni::JNIEnv;
use jni::errors::Result;
use jni::strings::JNIString;
use jni::objects::{JString, JValue};
use jni::sys::_jobject;

/// A wrapper for a java.lang.String
#[derive(Clone)]
pub struct JavaString<'a> {
    pub inner:  Object<'a>,
    env:        &'a JNIEnv<'a>
}

#[allow(clippy::from_over_into)]
impl<'a> Into<JValue<'a>> for JavaString<'a> {
    fn into(self) -> JValue<'a> {
        self.inner.clone().into()
    }
}

#[allow(clippy::from_over_into)]
impl<'a> Into<*mut _jobject> for JavaString<'a> {
    fn into(self) -> *mut _jobject {
        self.inner.inner.into_inner()
    }
}

impl<'a> JavaString<'a> {
    /// Create a JavaString wrapper. The caller must guarantee that the passed in Object is a java.lang.String and is not null.
    pub fn new(env: &'a JNIEnv<'a>, object: Object<'a>) -> Self {
        Self {
            inner: object,
            env
        }
    }

    /// Turn a Rust String into a JavaString
    pub fn from_rust<S: Into<JNIString>>(env: &'a JNIEnv<'a>, s: S) -> Result<Self> {
        let string = env.new_string(s)?;
        Ok(Self::new(env, Object::new(env, string.into(), Class::String(env)?)))
    }

    /// Turn a JavaString into a Rust String
    pub fn into_rust(&self) -> Result<String> {
        Ok(self.env.get_string(JString::from(self.inner.inner))?.into())
    }
}