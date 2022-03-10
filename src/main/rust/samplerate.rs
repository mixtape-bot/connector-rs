use jni::JNIEnv;
use jni::objects::{JClass, ReleaseMode};
use jni::sys::{jboolean, jdouble, jfloatArray, jint, jintArray, jlong};
use libsamplerate_sys::*;
use log::debug;

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_create(
    _: JNIEnv,
    _: JClass,
    src_type: jint,
    channels: jint,
) -> jlong {
    debug!("(samplerate) create, src_type: {}, channels: {}", src_type, channels);

    let mut error = 0;
    let handle = src_new(src_type, channels, &mut error) as jlong;
    debug!("(samplerate) new: {}", handle);

    handle
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    instance: jlong,
) {
    debug!("(samplerate) destroy, handle: {}", instance);

    let handle = instance as *mut SRC_STATE;

    /* destroy given instance */
    src_delete(handle);
    std::mem::drop(handle);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_reset(
    _: JNIEnv,
    _: JClass,
    instance: jlong,
) {
    debug!("(samplerate) reset, handle: {}", instance);
    src_reset(instance as *mut SRC_STATE);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_samplerate_SampleRateLibrary_process(
    env: JNIEnv,
    _: JClass,
    instance: jlong,
    input_array: jfloatArray,
    input_offset: jint,
    input_length: jint,
    output_array: jfloatArray,
    output_offset: jint,
    output_length: jint,
    eof: jboolean,
    source_ratio: jdouble,
    progress: jintArray,
) -> jint {
    debug!(
        "(samplerate) process, handle: {}, output_length: {}, output_offset: {}, input_length: {}, input_offset: {}, source_ratio: {}, is eof: {}",
        instance,
        output_length,
        output_offset,
        input_length,
        input_offset,
        source_ratio,
        eof,
    );

    /* input */
    let in_auto_ptr = env
        .get_float_array_elements(input_array, ReleaseMode::CopyBack)
        .expect("Unable to get input array.");

    let in_ptr = in_auto_ptr.as_ptr();
    let in_size = in_auto_ptr.size().unwrap() as usize;
    let input = Vec::from_raw_parts(in_ptr as *mut f32, in_size, in_size);

    /* output */
    let out_auto_ptr = env
        .get_float_array_elements(output_array, ReleaseMode::CopyBack)
        .expect("Unable to get output array.");

    let out_ptr = out_auto_ptr.as_ptr();
    let out_size = out_auto_ptr.size().unwrap() as usize;
    let mut output = Vec::from_raw_parts(out_ptr as *mut f32, out_size, out_size);

    let mut src_data = SRC_DATA {
        data_in: input[input_offset as usize..].as_ptr(),
        input_frames: input_length as i64,
        input_frames_used: 0,
        end_of_input: eof as i32,
        data_out: output[output_offset as usize..].as_mut_ptr(),
        output_frames: output_length as i64,
        output_frames_gen: 0,
        src_ratio: source_ratio,
    };

    let result = src_process(instance as *mut SRC_STATE, &mut src_data);
    let prog = [src_data.input_frames_used as jint, src_data.output_frames_gen as jint];

    env
        .set_int_array_region(progress, 0, &prog)
        .expect("Unable to write to progress array.");

    std::mem::forget(input);
    std::mem::forget(output);

    result
}
