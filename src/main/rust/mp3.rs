use crate::util::get_direct_buffer_address;
use jni::sys::{jlong, jint, jobject};
use jni::objects::JClass;
use jni::JNIEnv;
use log::debug;
use mpg123_sys::*;

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_create(_: JNIEnv, _: JClass) -> jlong {
    debug!("(mp3) create");

    mpg123_init();

    let handle = mpg123_new(std::ptr::null_mut(), std::ptr::null_mut());
    // TODO: try to check if this is a null handle.

    if mpg123_open_feed(handle) != 0 {
        mpg123_delete(handle);
        return 0
    };

    handle as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_destroy(_: JNIEnv, _: JClass, instance: jlong) {
    debug!("(mp3) destroy, instance: {}", instance);

    let handle = instance as *mut mpg123_handle;

    mpg123_close(handle);
    mpg123_delete(handle);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_mp3_Mp3DecoderLibrary_decode(
    jni: JNIEnv,
    _: JClass,
    instance: jlong,
    input_buffer: jobject,
    input_length: jint,
    output_buffer: jobject,
    output_length: jint,
) -> jlong {
    debug!("(mp3) decode, instance: {}, input_length: {}, output_length: {}", instance, input_length, output_length);

    if instance == 0 {
        return -1;
    };

    /* get input/output */
    let input = get_direct_buffer_address(jni, input_buffer)
        .expect("Unable to get input buffer.")
        .as_ptr();

    let output = get_direct_buffer_address(jni, output_buffer)
        .expect("Unable to get input buffer.")
        .as_mut_ptr();

    let mut used_bytes = 0;
    let result = mpg123_decode(instance as *mut mpg123_handle, input, input_length as usize, output, output_length as usize, &mut used_bytes) as jlong;
    if result != 0 {
        if result > 0 {
            return -(result + 100)
        } else {
            return result
        }
    }

    used_bytes as jlong
}
