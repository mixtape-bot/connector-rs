use std::mem::size_of;

use fdk_aac_sys::{AACDEC_FLUSH, aacDecoder_Close, aacDecoder_ConfigRaw, aacDecoder_DecodeFrame, aacDecoder_Fill, aacDecoder_GetStreamInfo, aacDecoder_Open, HANDLE_AACDECODER};
use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass};
use jni::sys::{jboolean, jint, jlong, jobject};
use log::debug;

use crate::util::get_direct_short_buffer_address;

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_create(
    _: JNIEnv,
    _: JClass,
    transport_type: jint,
) -> jlong {
    debug!("(aac) open, transport_type: {}", transport_type);
    aacDecoder_Open(transport_type, 1) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
) {
    debug!("(aac) destroy, decoder: {}.", handle);
    aacDecoder_Close(handle as HANDLE_AACDECODER)
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_configure(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
    buffer_data: jlong,
) -> jint {
    debug!("(aac) configure, decoder: {}, buffer: {:?}, buffer_size: {}.", handle, buffer_data, 8);

    let buffer_size = size_of::<jlong>();
    debug!("(aac) configure, hi");
    let mut buffer_ptr = Box::into_raw(Box::new(buffer_data)) as *mut u8;
    debug!("(aac) configure, hi");

    aacDecoder_ConfigRaw(handle as HANDLE_AACDECODER, &mut buffer_ptr as *mut _, &(buffer_size as u32)) as jint
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_fill(
    jni: JNIEnv,
    _: JClass,
    handle: jlong,
    buffer: JByteBuffer,
    buffer_offset: jint,
    buffer_length: jint,
) -> jint {
    let input = jni
        .get_direct_buffer_address(buffer)
        .unwrap();

    let length = buffer_length as u32;
    let offset = buffer_offset as u32;
    let mut buffer_valid_length = (length - offset) as u32;

    debug!("(aac) fill, decoder: {}, buffer_offset: {}, buffer_length: {}", handle, offset, length);
    aacDecoder_Fill(
        handle as HANDLE_AACDECODER,
        &mut input.as_mut_ptr(),
        &length,
        &mut buffer_valid_length,
    );

    let used = (length - offset - buffer_valid_length) as jint;
    debug!("(aac) fill, used {}", used);

    return used;
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_decode(
    jni: JNIEnv,
    _: JClass,
    handle: jlong,
    buffer: jobject,
    buffer_length: jint,
    flush: jboolean,
) -> jint {
    let output = get_direct_short_buffer_address(jni, buffer)
        .unwrap();

    let flush: bool = std::mem::transmute(flush);

    debug!("(aac) decode, decoder: {}, buffer_length: {}, flush: {}", handle, buffer_length, flush);

    aacDecoder_DecodeFrame(
        handle as HANDLE_AACDECODER,
        output.as_mut_ptr(),
        buffer_length,
        if flush { AACDEC_FLUSH } else { 0 },
    ) as jint
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_aac_AacDecoderLibrary_getStreamInfo(
    _: JNIEnv,
    _: JClass,
    handle: jlong,
) -> jlong {
    let stream_info = &*aacDecoder_GetStreamInfo(handle as HANDLE_AACDECODER);

    let sample_rate = stream_info.sampleRate as jlong;
    let frame_size = stream_info.frameSize as jlong;
    let num_channels = stream_info.numChannels as jlong;

    sample_rate << 32u64 | frame_size << 16 | num_channels
}
