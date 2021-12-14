pub mod class;
pub use class::*;

pub mod object;
pub use object::*;

pub mod primitives;
pub use primitives::*;

pub mod abstractions;
pub use abstractions::*;

pub use jni::*;

#[cfg(test)]
mod test {
    use jni::{JavaVM, JNIVersion, InitArgsBuilder};
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