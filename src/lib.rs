pub mod class;
pub mod object;
pub mod primitives;
pub mod abstractions;

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