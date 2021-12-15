use jni::JNIEnv;
use crate::{Class, JavaString, Map, Object};
use jni::errors::Result;
use jni::objects::JValue;

/// Wrapper around `java.lang.System`
pub struct System<'a> {
    env: &'a JNIEnv<'a>
}

impl<'a> System<'a> {
    /// Create a new System abstraction
    pub fn new(env: &'a JNIEnv<'a>) -> Self {
        Self {
            env
        }
    }

    /// Removes the system property indicated by the specified key.
    pub fn clear_property<S: AsRef<str>>(&self, key: S) -> Result<Option<String>> {
        let key_jstring = JavaString::from_rust(self.env, key)?;
        let class = Class::System(self.env)?.class;
        let prop = self.env.call_static_method(class, "clearProperty", "(Ljava/lang/String;)Ljava/lang/String;", &[key_jstring.into()])?.l()?;
        if prop.is_null() {
            return Ok(None);
        }

        let prop = JavaString::new(self.env, Object::new(self.env, prop, Class::String(self.env)?));

        Ok(Some(prop.into_rust()?))
    }

    /// Returns the current time in milliseconds.
    pub fn current_time_millis(&self) -> Result<i64> {
        let class = Class::System(self.env)?.class;
        let value = self.env.call_static_method(class, "currentTimeMillis", "()J", &[])?;
        value.j()
    }

    /// Terminates the currently running Java Virtual Machine.
    pub fn exit(&self, status: i32) -> Result<()> {
        let class = Class::System(self.env)?.class;
        self.env.call_static_method(class, "exit", "(I)V", &[JValue::Int(status)])?;
        Ok(())
    }

    /// Runs the garbage collector.
    pub fn gc(&self) -> Result<()> {
        let class = Class::System(self.env)?.class;
        self.env.call_static_method(class, "gc", "()V", &[])?;
        Ok(())
    }

    /// Returns an unmodifiable string map view of the current system environment.
    pub fn get_env(&self) -> Result<Map<'a>> {
        let class = Class::System(self.env)?.class;
        let value = self.env.call_static_method(class, "getenv", "()Ljava/util/Map;", &[])?.l()?;

        let object = Object::new(self.env, value, Class::Map(self.env)?);
        let string_class = Class::String(self.env)?;
        let map = Map::new(self.env, object, string_class.clone(), string_class);
        Ok(map)
    }

    /// Gets the value of the specified environment variable.
    pub fn get_env_with_name<S: AsRef<str>>(&self, name: S) -> Result<Option<String>> {
        let jstring = JavaString::from_rust(self.env, name)?;
        let class = Class::System(self.env)?;
        let value = self.env.call_static_method(class, "getenv", "(Ljava/lang/String;)Ljava/lang/String;", &[jstring.into()])?.l()?;

        if value.is_null() {
            return Ok(None);
        }

        let object = Object::new(self.env, value, Class::String(self.env)?);
        let string = JavaString::new(self.env, object);
        Ok(Some(string.into_rust()?))
    }

    /// Gets the system property indicated by the specified key.
    pub fn get_property<S: AsRef<str>>(&self, key: S) -> Result<Option<String>> {
        let jstring = JavaString::from_rust(self.env, key)?;
        let class = Class::System(self.env)?.class;
        let value = self.env.call_static_method(class, "getProperty", "(Ljava/lang/String;)Ljava/lang/String;", &[jstring.into()])?.l()?;
        if value.is_null() {
            return Ok(None);
        }

        let object = Object::new(self.env, value, Class::String(self.env)?);
        let string = JavaString::new(self.env, object);
        Ok(Some(string.into_rust()?))
    }

    /// Returns the system-dependent line separator string.
    pub fn line_separator(&self) -> Result<String> {
        let class = Class::System(self.env)?.class;
        let value = self.env.call_static_method(class, "lineSeparator", "()Ljava/lang/String;", &[])?.l()?;

        let object = Object::new(self.env, value, Class::String(self.env)?);
        let string = JavaString::new(self.env, object);
        string.into_rust()
    }

    /// Loads a code file with the specified filename from the local file system as a dynamic library.
    pub fn load<S: AsRef<str>>(&self, filename: S) -> Result<()> {
        let jstring = JavaString::from_rust(self.env, filename)?;
        let class = Class::System(self.env)?.class;
        self.env.call_static_method(class, "load", "(Ljava/lang/String;)V", &[jstring.into()])?;
        Ok(())
    }

    /// Loads the system library specified by the libname argument.
    pub fn load_library<S: AsRef<str>>(&self, libname: S) -> Result<()> {
        let jstring = JavaString::from_rust(self.env, libname)?;
        let class = Class::System(self.env)?.class;
        self.env.call_static_method(class, "loadLibrary", "(Ljava/lang/String;)V", &[jstring.into()])?;
        Ok(())
    }

    /// Returns the current value of the running Java Virtual Machine's high-resolution time source, in nanoseconds.
    pub fn nano_time(&self) -> Result<i64> {
        let class = Class::System(self.env)?.class;
        let value = self.env.call_static_method(class, "nanoTime", "()J", &[])?;
        value.j()
    }

    /// Runs the finalization methods of any objects pending finalization.
    pub fn run_finalization(&self) -> Result<()> {
        let class = Class::System(self.env)?.class;
        self.env.call_static_method(class, "runFinalization", "()V", &[])?;
        Ok(())
    }

    /// Sets the system property indicated by the specified key.
    pub fn set_property<A: AsRef<str>, B: AsRef<str>>(&self, key: A, value: B) -> Result<Option<String>> {
        let key = JavaString::from_rust(self.env, key)?;
        let value = JavaString::from_rust(self.env, value)?;
        let class = Class::System(self.env)?.class;

        let value = self.env.call_static_method(class, "setProperty", "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;", &[key.into(), value.into()])?.l()?;
        if value.is_null() {
            return Ok(None);
        }

        let object = Object::new(self.env, value, Class::String(self.env)?);
        let string = JavaString::new(self.env, object);
        Ok(Some(string.into_rust()?))
    }
}
