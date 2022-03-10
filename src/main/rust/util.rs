use std::os::raw::c_void;
use std::slice;
use jni::JNIEnv;
use jni::errors::Result;
use jni::sys::jobject;

pub fn get_direct_short_buffer_address<'a>(jni: JNIEnv, buf: jobject) -> Result<&'a mut [i16]> {
    let ptr: *mut c_void = jni_unchecked!(jni.get_native_interface(), GetDirectBufferAddress, buf);
    let capacity = jni_unchecked!(jni.get_native_interface(), GetDirectBufferCapacity, buf);

    unsafe { Ok(slice::from_raw_parts_mut(ptr as *mut i16, capacity as usize)) }
}
pub fn get_direct_buffer_address<'a>(jni: JNIEnv, buf: jobject) -> Result<&'a mut [u8]> {
    let ptr: *mut c_void = jni_unchecked!(jni.get_native_interface(), GetDirectBufferAddress, buf);
    let capacity = jni_unchecked!(jni.get_native_interface(), GetDirectBufferCapacity, buf);

    unsafe { Ok(slice::from_raw_parts_mut(ptr as *mut u8, capacity as usize)) }
}
