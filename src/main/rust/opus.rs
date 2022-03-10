use audiopus_sys::{OPUS_OK, opus_decoder_create, opus_decoder_destroy, opus_decode, opus_encoder_create, opus_encoder_destroy, opus_encode, OpusDecoder, OPUS_SET_COMPLEXITY_REQUEST, opus_encoder_ctl, OpusEncoder};
use jni::objects::{JByteBuffer, JClass};
use jni::sys::{jint, jlong, jobject};
use jni::JNIEnv;
use log::debug;
use crate::util::get_direct_short_buffer_address;

// decoder
#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_create(
    _: JNIEnv,
    _: JClass,
    sample_rate: jint,
    channel_count: jint,
) -> jlong {
     debug!("(opus) decoder:create, sample_rate: {}, channel_count: {}", sample_rate, channel_count);

    /* create the decoder */
    let mut opus_code = 0;
    let decoder = opus_decoder_create(sample_rate, channel_count, &mut opus_code);

    /* check for errors. */
    if opus_code == OPUS_OK || !decoder.is_null() {
        /* return the pointer? */
        return decoder as jlong
    };

    opus_code as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    decoder_ptr: jlong,
) {
    debug!("(opus) decoder:destroy, instance: {}", decoder_ptr);

    let decoder = Box::leak(Box::from_raw(decoder_ptr as *mut OpusDecoder));
    opus_decoder_destroy(decoder)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusDecoderLibrary_decode(
    env: JNIEnv,
    _: JClass,
    decoder_ptr: jlong,
    input_buffer: JByteBuffer,
    input_size: jint,
    output_buffer: jobject,
    frame_size: jint
) -> jint {
    debug!("(opus) decoder:decode, instance: {}, input_size: {}, frame_size: {}", decoder_ptr, input_size, frame_size);

    /* get the decoder */
    let decoder = Box::leak(Box::from_raw(decoder_ptr as *mut OpusDecoder));

    /* get the input/output buffers */
    let input = env.get_direct_buffer_address(input_buffer)
        .expect("Unable to resolve input buffer address.");

    let output = get_direct_short_buffer_address(env, output_buffer)
        .expect("Unable to resolve output buffer address.");

    opus_decode(decoder, input.as_ptr(), input_size, output.as_mut_ptr(), frame_size, 0) as i32
}

// encoder
#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_create(
    _: JNIEnv,
    _: JClass,
    sample_rate: jint,
    channel_count: jint,
    application: jint,
    quality: jint,
) -> jlong {
    debug!("(opus) encoder:create, sample_rate: {}, channel_count: {}, application: {}, quality: {}", sample_rate, channel_count, application, quality);

    /* create the encoder. */
    let mut opus_code = 0;
    let encoder = opus_encoder_create(sample_rate, channel_count, application, &mut opus_code);

    /* check for errors. */
    if opus_code == OPUS_OK || !encoder.is_null() {
        opus_encoder_ctl(encoder, OPUS_SET_COMPLEXITY_REQUEST, quality);
        return encoder as jlong
    };

    opus_code as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    encoder_ptr: jlong
) {
    debug!("(opus) encoder:destroy, instance: {}", encoder_ptr);

    let encoder = Box::leak(Box::from_raw(encoder_ptr as *mut OpusEncoder));
    opus_encoder_destroy(encoder)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_opus_OpusEncoderLibrary_encode(
    jni: JNIEnv,
    _: JClass,
    encoder_ptr: jlong,
    input_buffer: jobject,
    frame_size: jint,
    output_buffer: JByteBuffer,
    output_capacity: jint
) -> jint {
    debug!("(opus) encoder:encode, instance: {}, frame_size: {}, output_capacity: {}", encoder_ptr, frame_size, output_capacity);

    /* get the decoder */
    let encoder = Box::leak(Box::from_raw(encoder_ptr as *mut OpusEncoder));

    /* get the input/output buffers */
    let input_ptr = get_direct_short_buffer_address(jni, input_buffer)
        .expect("Unable to resolve input buffer address.")
        .as_ptr();

    let output_ptr = jni.get_direct_buffer_address(output_buffer)
        .expect("Unable to resolve output buffer address.")
        .as_mut_ptr();

    /* decode */
    opus_encode(encoder, input_ptr, frame_size, output_ptr, output_capacity)
}
