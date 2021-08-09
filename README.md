# eJNI
eJNI is a Rust crate to make working with Java's JNI less painful by providing abstractions.

eJNI provides abstractions for often-used classes from the Java standard library, like Map and List.
Besides this eJNI also provides easy ways to work with Java's primitives and their object counterparts (e.g `int` and `Integer`).

## API stability
eJNI is a young library, and it's API is likely to change. The code in the library is fully tested though.

## Examples
### ArrayList
The following example shows how to create an `ArrayList<String>`, fill it with 10 Strings, and return it to Java.
```rs
use ejni::{List, Class, Object, JavaString};
use jni::sys::jobject;

#[no_mangle]
pub fn Java_MyClass_doNative(env: JNIEnv<'_>, _: JClass) -> jobject {
    // Create a new java.util.ArrayList containing java.lang.String's
    let list = List::arraylist(&env, Class::String(&env).unwrap()).unwrap();

    // Add 10 Strings to the List
    for i in 0..10 {
        let string = JavaString::from_rust(&env, format!("Iteration {}", i)).unwrap();
        list.add(&env, string).unwrap();
    }

    list.into()
}
```