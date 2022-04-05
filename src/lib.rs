//! eJNI

#![deny(clippy::missing_safety_doc)]
#![warn(missing_docs)]

mod class;
pub use class::*;

mod object;
pub use object::*;

mod primitives;
pub use primitives::*;

mod abstractions;
pub use abstractions::*;

#[cfg(test)]
mod test {
    use jni::{InitArgsBuilder, JNIVersion, JavaVM};
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    lazy_static! {
        pub static ref JVM: Mutex<JavaVM> = {
            let jvm_args = InitArgsBuilder::new()
                .version(JNIVersion::V8)
                .option("-Xcheck:jni")
                .build()
                .unwrap();

            Mutex::new(JavaVM::new(jvm_args).unwrap())
        };
    }
}
