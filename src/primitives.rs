use jni::errors::Result;
use jni::sys::{
    jbooleanArray, jbyteArray, jcharArray, jdoubleArray, jfloatArray, jintArray, jlongArray,
    jshortArray,
};
use jni::JNIEnv;

/// Helper struct for Java's Primitives
pub struct Primitive();

impl Primitive {
    /// Create a new int[]
    pub fn new_int_array(env: &JNIEnv<'_>, ints: &[i32]) -> Result<jintArray> {
        let arr = env.new_int_array(ints.len() as i32)?;
        env.set_int_array_region(arr, 0, ints)?;
        Ok(arr)
    }

    /// Create a new byte[]
    pub fn new_byte_array(env: &JNIEnv<'_>, bytes: &[u8]) -> Result<jbyteArray> {
        // SAFETY: The compiler guarantees safety here.
        let bytes: &[i8] = unsafe { std::mem::transmute(bytes) };
        let arr = env.new_byte_array(bytes.len() as i32)?;
        env.set_byte_array_region(arr, 0, bytes)?;
        Ok(arr)
    }

    /// Create a new long[]
    pub fn new_long_array(env: &JNIEnv<'_>, longs: &[i64]) -> Result<jlongArray> {
        let arr = env.new_long_array(longs.len() as i32)?;
        env.set_long_array_region(arr, 0, longs)?;
        Ok(arr)
    }

    /// Create a new float[]
    pub fn new_float_array(env: &JNIEnv<'_>, floats: &[f32]) -> Result<jfloatArray> {
        let arr = env.new_float_array(floats.len() as i32)?;
        env.set_float_array_region(arr, 0, floats)?;
        Ok(arr)
    }

    /// Create a new double[]
    pub fn new_double_array(env: &JNIEnv<'_>, doubles: &[f64]) -> Result<jdoubleArray> {
        let arr = env.new_double_array(doubles.len() as i32)?;
        env.set_double_array_region(arr, 0, doubles)?;
        Ok(arr)
    }

    /// Create a new char[]
    pub fn new_char_array(env: &JNIEnv<'_>, chars: &[u16]) -> Result<jcharArray> {
        let arr = env.new_char_array(chars.len() as i32)?;
        env.set_char_array_region(arr, 0, chars)?;
        Ok(arr)
    }

    /// Create a new short[]
    pub fn new_short_array(env: &JNIEnv<'_>, shorts: &[i16]) -> Result<jshortArray> {
        let arr = env.new_short_array(shorts.len() as i32)?;
        env.set_short_array_region(arr, 0, shorts)?;
        Ok(arr)
    }

    /// Create a new boolean[]
    pub fn new_boolean_array(env: &JNIEnv<'_>, booleans: &[bool]) -> Result<jbooleanArray> {
        // SAFETY: The compiler guarantees safety here.
        let booleans: &[u8] = unsafe { std::mem::transmute(booleans) };
        let arr = env.new_boolean_array(booleans.len() as i32)?;
        env.set_boolean_array_region(arr, 0, booleans)?;
        Ok(arr)
    }

    /// Copy an int[] to a Vec.
    pub fn get_int_array(env: &JNIEnv<'_>, ints: jintArray) -> Result<Vec<i32>> {
        let len = env.get_array_length(ints)?;
        let mut buf = vec![0; len as usize];
        env.get_int_array_region(ints, 0, &mut buf)?;
        Ok(buf)
    }

    /// Copy a byte[] to a Vec
    pub fn get_byte_array(env: &JNIEnv<'_>, bytes: jbyteArray) -> Result<Vec<u8>> {
        let len = env.get_array_length(bytes)?;
        let mut buf = vec![0; len as usize];
        env.get_byte_array_region(bytes, 0, &mut buf)?;

        let buf: Vec<_> = buf.into_iter().map(|f| f as u8).collect();

        Ok(buf)
    }

    /// Copy a long[] to a Vec
    pub fn get_long_array(env: &JNIEnv<'_>, longs: jlongArray) -> Result<Vec<i64>> {
        let len = env.get_array_length(longs)?;
        let mut buf = vec![0; len as usize];
        env.get_long_array_region(longs, 0, &mut buf)?;
        Ok(buf)
    }

    /// Copy a float[] to a Vec
    pub fn get_float_array(env: &JNIEnv<'_>, floats: jfloatArray) -> Result<Vec<f32>> {
        let len = env.get_array_length(floats)?;
        let mut buf = vec![0f32; len as usize];
        env.get_float_array_region(floats, 0, &mut buf)?;
        Ok(buf)
    }

    /// Copy a double[] to a Vec
    pub fn get_double_array(env: &JNIEnv<'_>, doubles: jdoubleArray) -> Result<Vec<f64>> {
        let len = env.get_array_length(doubles)?;
        let mut buf = vec![0f64; len as usize];
        env.get_double_array_region(doubles, 0, &mut buf)?;
        Ok(buf)
    }

    /// Copy a char[] to a Vec
    pub fn get_char_array(env: &JNIEnv<'_>, chars: jcharArray) -> Result<Vec<u16>> {
        let len = env.get_array_length(chars)?;
        let mut buf = vec![0; len as usize];
        env.get_char_array_region(chars, 0, &mut buf)?;
        Ok(buf)
    }

    /// Copy a short[] to a Vec
    pub fn get_short_array(env: &JNIEnv<'_>, shorts: jshortArray) -> Result<Vec<i16>> {
        let len = env.get_array_length(shorts)?;
        let mut buf = vec![0; len as usize];
        env.get_short_array_region(shorts, 0, &mut buf)?;
        Ok(buf)
    }

    /// Copy a boolean[] to a Vec
    pub fn get_boolean_array(env: &JNIEnv<'_>, booleans: jbooleanArray) -> Result<Vec<bool>> {
        let len = env.get_array_length(booleans)?;
        let mut buf = vec![0; len as usize];
        env.get_boolean_array_region(booleans, 0, &mut buf)?;

        let buf: Vec<_> = buf.into_iter().map(|f| f == 1).collect();

        Ok(buf)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::JVM;

    #[test]
    fn new_int_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        assert!(Primitive::new_int_array(&env, value).is_ok());
    }

    #[test]
    fn new_byte_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        assert!(Primitive::new_byte_array(&env, value).is_ok());
    }

    #[test]
    fn new_long_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        assert!(Primitive::new_long_array(&env, value).is_ok());
    }

    #[test]
    fn new_float_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1f32, 2f32, 3f32];
        assert!(Primitive::new_float_array(&env, value).is_ok());
    }

    #[test]
    fn new_double_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1f64, 2f64, 3f64];
        assert!(Primitive::new_double_array(&env, value).is_ok());
    }

    #[test]
    fn new_char_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        assert!(Primitive::new_char_array(&env, value).is_ok());
    }

    #[test]
    fn new_short_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        assert!(Primitive::new_short_array(&env, value).is_ok());
    }

    #[test]
    fn new_boolean_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[true, false, true];
        assert!(Primitive::new_boolean_array(&env, value).is_ok());
    }

    #[test]
    fn get_int_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        let arr = Primitive::new_int_array(&env, value).unwrap();

        let converted = Primitive::get_int_array(&env, arr);
        assert!(converted.is_ok());
        let converted = converted.unwrap();

        assert_eq!(value, converted.as_slice());
    }

    #[test]
    fn get_byte_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        let arr = Primitive::new_byte_array(&env, value).unwrap();

        let converted = Primitive::get_byte_array(&env, arr);
        assert!(converted.is_ok());
        let converted = converted.unwrap();

        assert_eq!(value, converted.as_slice());
    }

    #[test]
    fn get_long_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        let arr = Primitive::new_long_array(&env, value).unwrap();

        let converted = Primitive::get_long_array(&env, arr);
        assert!(converted.is_ok());
        let converted = converted.unwrap();

        assert_eq!(value, converted.as_slice());
    }

    #[test]
    fn get_float_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1f32, 2f32, 3f32];
        let arr = Primitive::new_float_array(&env, value).unwrap();

        let converted = Primitive::get_float_array(&env, arr);
        assert!(converted.is_ok());
        let converted = converted.unwrap();

        assert_eq!(value, converted.as_slice());
    }

    #[test]
    fn get_double_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1f64, 2f64, 3f64];
        let arr = Primitive::new_double_array(&env, value).unwrap();

        let converted = Primitive::get_double_array(&env, arr);
        assert!(converted.is_ok());
        let converted = converted.unwrap();

        assert_eq!(value, converted.as_slice());
    }

    #[test]
    fn get_char_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        let arr = Primitive::new_char_array(&env, value).unwrap();

        let converted = Primitive::get_char_array(&env, arr);
        assert!(converted.is_ok());
        let converted = converted.unwrap();

        assert_eq!(value, converted.as_slice());
    }

    #[test]
    fn get_short_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[1, 2, 3];
        let arr = Primitive::new_short_array(&env, value).unwrap();

        let converted = Primitive::get_short_array(&env, arr);
        assert!(converted.is_ok());
        let converted = converted.unwrap();

        assert_eq!(value, converted.as_slice());
    }

    #[test]
    fn get_boolean_array() {
        let jvm = JVM.lock().unwrap();
        let env = jvm.attach_current_thread().unwrap();

        let value = &[true, false, true];
        let arr = Primitive::new_boolean_array(&env, value).unwrap();

        let converted = Primitive::get_boolean_array(&env, arr);
        assert!(converted.is_ok());
        let converted = converted.unwrap();

        assert_eq!(value, converted.as_slice());
    }
}
