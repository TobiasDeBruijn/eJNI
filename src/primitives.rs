use jni::sys::{jintArray, jbyteArray, jlongArray, jfloatArray, jdoubleArray, jcharArray, jshortArray, jbooleanArray};
use jni::JNIEnv;
use jni::errors::Result;

pub struct Primitive();

impl<'a> Primitive {
    pub fn int_array(env: JNIEnv<'a>, ints: &'a [i32]) -> Result<jintArray> {
        let arr = env.new_int_array(ints.len() as i32)?;
        env.set_int_array_region(arr, 0, ints)?;
        Ok(arr)
    }

    pub fn byte_array(env: JNIEnv<'a>, bytes: &'a [u8]) -> Result<jbyteArray> {
        // SAFETY: The compiler guarantees safety here.
        let bytes: &[i8] = unsafe { std::mem::transmute(bytes) };
        let arr = env.new_byte_array(bytes.len() as i32)?;
        env.set_byte_array_region(arr, 0, bytes)?;
        Ok(arr)
    }

    pub fn long_array(env: JNIEnv<'a>, longs: &'a [i64]) -> Result<jlongArray> {
        let arr = env.new_long_array(longs.len() as i32)?;
        env.set_long_array_region(arr, 0, longs)?;
        Ok(arr)
    }

    pub fn float_array(env: JNIEnv<'a>, floats: &'a [f32]) -> Result<jfloatArray> {
        let arr = env.new_float_array(floats.len() as i32)?;
        env.set_float_array_region(arr, 0, floats)?;
        Ok(arr)
    }

    pub fn double_array(env: JNIEnv<'a>, doubles: &'a [f64]) -> Result<jdoubleArray> {
        let arr = env.new_double_array(doubles.len() as i32)?;
        env.set_double_array_region(arr, 0, doubles)?;
        Ok(arr)
    }

    pub fn char_array(env: JNIEnv<'a>, chars: &'a [u16]) -> Result<jcharArray> {
        let arr = env.new_char_array(chars.len() as i32)?;
        env.set_char_array_region(arr, 0, chars)?;
        Ok(arr)
    }

    pub fn short_array(env: JNIEnv<'a>, shorts: &'a [i16]) -> Result<jshortArray> {
        let arr = env.new_short_array(shorts.len() as i32)?;
        env.set_short_array_region(arr, 0, shorts)?;
        Ok(arr)
    }

    pub fn boolean_array(env: JNIEnv<'a>, booleans: &'a [bool]) -> Result<jbooleanArray> {
        // SAFETY: The compiler guarantees safety here.
        let booleans: &[u8] = unsafe { std::mem::transmute(booleans) };
        let arr = env.new_boolean_array(booleans.len() as i32)?;
        env.set_boolean_array_region(arr, 0, booleans)?;
        Ok(arr)
    }
}