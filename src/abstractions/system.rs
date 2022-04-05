use crate::{Class, JavaString, Map, Object};
use jni::errors::Result;
use jni::objects::JValue;
use jni::JNIEnv;

/// Wrapper around `java.lang.System`
pub struct System<'a> {
    env: &'a JNIEnv<'a>,
}

impl<'a> System<'a> {
    /// Create a new System abstraction
    pub fn new(env: &'a JNIEnv<'a>) -> Self {
        Self { env }
    }

    /// Removes the system property indicated by the specified key.
    pub fn clear_property<S: AsRef<str>>(&self, key: S) -> Result<Option<String>> {
        let key_jstring = JavaString::from_rust(self.env, key)?;
        let class = Class::System(self.env)?.class;
        let prop = self
            .env
            .call_static_method(
                class,
                "clearProperty",
                "(Ljava/lang/String;)Ljava/lang/String;",
                &[key_jstring.into()],
            )?
            .l()?;
        if prop.is_null() {
            return Ok(None);
        }

        let prop = JavaString::new(
            self.env,
            Object::new(self.env, prop, Class::String(self.env)?),
        );

        Ok(Some(prop.into_rust()?))
    }

    /// Returns the current time in milliseconds.
    pub fn current_time_millis(&self) -> Result<i64> {
        let class = Class::System(self.env)?.class;
        let value = self
            .env
            .call_static_method(class, "currentTimeMillis", "()J", &[])?;
        value.j()
    }

    /// Terminates the currently running Java Virtual Machine.
    pub fn exit(&self, status: i32) -> Result<()> {
        let class = Class::System(self.env)?.class;
        self.env
            .call_static_method(class, "exit", "(I)V", &[JValue::Int(status)])?;
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
        let value = self
            .env
            .call_static_method(class, "getenv", "()Ljava/util/Map;", &[])?
            .l()?;

        let object = Object::new(self.env, value, Class::Map(self.env)?);
        let string_class = Class::String(self.env)?;
        let map = Map::new(self.env, object, string_class.clone(), string_class);
        Ok(map)
    }

    /// Gets the value of the specified environment variable.
    pub fn get_env_with_name<S: AsRef<str>>(&self, name: S) -> Result<Option<String>> {
        let jstring = JavaString::from_rust(self.env, name)?;
        let class = Class::System(self.env)?;
        let value = self
            .env
            .call_static_method(
                class,
                "getenv",
                "(Ljava/lang/String;)Ljava/lang/String;",
                &[jstring.into()],
            )?
            .l()?;

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
        let value = self
            .env
            .call_static_method(
                class,
                "getProperty",
                "(Ljava/lang/String;)Ljava/lang/String;",
                &[jstring.into()],
            )?
            .l()?;
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
        let value = self
            .env
            .call_static_method(class, "lineSeparator", "()Ljava/lang/String;", &[])?
            .l()?;

        let object = Object::new(self.env, value, Class::String(self.env)?);
        let string = JavaString::new(self.env, object);
        string.into_rust()
    }

    /// Loads a code file with the specified filename from the local file system as a dynamic library.
    pub fn load<S: AsRef<str>>(&self, filename: S) -> Result<()> {
        let jstring = JavaString::from_rust(self.env, filename)?;
        let class = Class::System(self.env)?.class;
        self.env
            .call_static_method(class, "load", "(Ljava/lang/String;)V", &[jstring.into()])?;
        Ok(())
    }

    /// Loads the system library specified by the libname argument.
    pub fn load_library<S: AsRef<str>>(&self, libname: S) -> Result<()> {
        let jstring = JavaString::from_rust(self.env, libname)?;
        let class = Class::System(self.env)?.class;
        self.env.call_static_method(
            class,
            "loadLibrary",
            "(Ljava/lang/String;)V",
            &[jstring.into()],
        )?;
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
        self.env
            .call_static_method(class, "runFinalization", "()V", &[])?;
        Ok(())
    }

    /// Sets the system property indicated by the specified key.
    pub fn set_property<A: AsRef<str>, B: AsRef<str>>(
        &self,
        key: A,
        value: B,
    ) -> Result<Option<String>> {
        let key = JavaString::from_rust(self.env, key)?;
        let value = JavaString::from_rust(self.env, value)?;
        let class = Class::System(self.env)?.class;

        let value = self
            .env
            .call_static_method(
                class,
                "setProperty",
                "(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;",
                &[key.into(), value.into()],
            )?
            .l()?;
        if value.is_null() {
            return Ok(None);
        }

        let object = Object::new(self.env, value, Class::String(self.env)?);
        let string = JavaString::new(self.env, object);
        Ok(Some(string.into_rust()?))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::JVM;

    #[test]
    fn clear_property() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        assert!(system.clear_property("Foo").is_ok());
    }

    #[test]
    fn current_time_millis() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        assert!(system.current_time_millis().is_ok());
    }

    #[test]
    fn gc() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        assert!(system.gc().is_ok());
    }

    #[test]
    fn get_env() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        assert!(system.get_env().is_ok());
    }

    #[test]
    fn get_env_with_name() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        assert!(system.get_env_with_name("Foo").is_ok());
    }

    #[test]
    fn get_property() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        assert!(system.get_property("Foo").is_ok());
    }

    #[test]
    fn line_separator() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        assert!(system.line_separator().is_ok());
    }

    #[test]
    fn load() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        #[cfg(unix)]
        {
            // Example of loading libc
            assert!(system.load("/lib/x86_64-linux-gnu/libc.so.6").is_ok());

            if env.exception_check().unwrap() {
                env.exception_clear().unwrap();
            }
        }
    }

    #[test]
    fn load_library() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        #[cfg(unix)]
        {
            // Example of trying to load libjava
            assert!(system.load_library("java").is_ok());

            if env.exception_check().unwrap() {
                env.exception_clear().unwrap();
            }
        }
    }

    #[test]
    fn nano_time() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        assert!(system.nano_time().is_ok());
    }

    #[test]
    fn set_property() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();
        let system = System::new(&env);

        assert!(system.set_property("Foo", "Bar").is_ok());

        let property_foo = system.get_property("Foo").unwrap();
        assert!(property_foo.is_some());
        assert_eq!(property_foo.unwrap(), "Bar");
    }
}
