use audiopus_sys::{OPUS_OK, opus_decoder_create, opus_decoder_destroy, opus_decode, opus_encoder_create, opus_encoder_destroy, opus_encode, OpusDecoder, OPUS_SET_COMPLEXITY_REQUEST, opus_encoder_ctl, OpusEncoder};
use jni::objects::{JByteBuffer, JClass};
use jni::sys::{jint, jlong, jobject};
use jni::JNIEnv;
use crate::util::get_direct_short_buffer_address;

// decoder
#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_create(
    _: JNIEnv,
    _: JClass,
    sample_rate: jint,
    channel_count: jint,
) -> jlong {
    /* create the decoder */
    let mut opus_code = 0;
    let decoder = unsafe { opus_decoder_create(sample_rate, channel_count, &mut opus_code) };

    /* check for errors. */
    if opus_code == OPUS_OK || !decoder.is_null() {
        /* return the pointer? */
        return decoder as jlong
    };

    opus_code as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    decoder_ptr: jlong,
) {
    let decoder = Box::leak(unsafe { Box::from_raw(decoder_ptr as *mut OpusDecoder) });
    unsafe { opus_decoder_destroy(decoder) }
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_decode(
    env: JNIEnv,
    _: JClass,
    decoder_ptr: jlong,
    input_buffer: JByteBuffer,
    input_size: jint,
    output_buffer: jobject,
    frame_size: jint
) -> jint {
    /* get the decoder */
    let decoder = Box::leak(unsafe { Box::from_raw(decoder_ptr as *mut OpusDecoder) });

    /* get the input/output buffers */
    let input = env.get_direct_buffer_address(input_buffer)
        .expect("Unable to resolve input buffer address.");

    let output = get_direct_short_buffer_address(env, output_buffer)
        .expect("Unable to resolve output buffer address.");

    unsafe { opus_decode(decoder, input.as_ptr(), input_size, output.as_mut_ptr(), frame_size, 0) as i32 }
}

// encoder
#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_create(
    _: JNIEnv,
    _: JClass,
    sample_rate: jint,
    channel_count: jint,
    application: jint,
    quality: jint,
) -> jlong {
    /* create the encoder. */
    let mut opus_code = 0;
    let encoder = unsafe { opus_encoder_create(sample_rate, channel_count, application, &mut opus_code) };

    /* check for errors. */
    if opus_code == OPUS_OK || !encoder.is_null() {
        unsafe { opus_encoder_ctl(encoder, OPUS_SET_COMPLEXITY_REQUEST, quality) };

        return encoder as jlong
    };

    opus_code as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    encoder_ptr: jlong
) {
    let encoder = Box::leak(unsafe { Box::from_raw(encoder_ptr as *mut OpusEncoder) });
    unsafe { opus_encoder_destroy(encoder) }
}

#[no_mangle]
pub extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_encode(
    jni: JNIEnv,
    _: JClass,
    encoder_ptr: jlong,
    input_buffer: jobject,
    frame_size: jint,
    output_buffer: JByteBuffer,
    output_capacity: jint
) -> jint {
    /* get the decoder */
    let encoder = Box::leak(unsafe { Box::from_raw(encoder_ptr as *mut OpusEncoder) });

    /* get the input/output buffers */
    let input_ptr = get_direct_short_buffer_address(jni, input_buffer)
        .expect("Unable to resolve input buffer address.")
        .as_ptr();

    let output_ptr = jni.get_direct_buffer_address(output_buffer)
        .expect("Unable to resolve output buffer address.")
        .as_mut_ptr();

    /* decode */
    unsafe { opus_encode(encoder, input_ptr, frame_size, output_ptr, output_capacity) }
}
