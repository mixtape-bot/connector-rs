use std::mem::MaybeUninit;
use std::os::raw::c_long;
use jni::JNIEnv;
use jni::objects::{JByteBuffer, JClass};
use jni::sys::{jfloatArray, jint, jlong, jobjectArray};
use log::debug;
use ogg_sys::ogg_packet;
use vorbis_sys::*;

type VorbisStateHandle = *mut VorbisState;

struct Pcm {
    internal: *mut *mut f32
}

impl Pcm {
    fn new() -> Self {
        Self { internal: unsafe { std::mem::zeroed() } }
    }

    fn pcm(&self, channel: usize, len: usize) -> &[f32] {
        unsafe { std::slice::from_raw_parts(*self.internal.offset(channel as isize) as *const f32, len) }
    }
}

struct VorbisState {
    info_ptr: *mut vorbis_info,
    block: MaybeUninit<vorbis_block>,
    dsp_state: MaybeUninit<vorbis_dsp_state>,
    initialized: bool
}

impl VorbisState {
    fn new() -> Self {
        let mut info = unsafe { std::mem::zeroed() };
        unsafe { vorbis_info_init(&mut info) };

        Self {
            info_ptr: to_ptr!(info),
            block: MaybeUninit::uninit(),
            dsp_state: MaybeUninit::uninit(),
            initialized: false
        }
    }

    fn from_ptr(ptr: VorbisStateHandle) -> &'static mut Self {
        from_ptr!(ptr)
    }

    fn get_info(&self) -> &mut vorbis_info {
        from_ptr!(self.info_ptr)
    }

    fn get_channel_count(&self) -> i32 {
        self.get_info().channels
    }
}

fn build_ogg_packet(
    env: JNIEnv,
    ogg_packet: &mut ogg_packet,
    buffer: JByteBuffer,
    offset: usize,
    length: i64,
    is_beginning: bool,
)  {
    let packet = env
        .get_direct_buffer_address(buffer)
        .expect("Unable to get packet");

    ogg_packet.bytes = length as c_long;
    ogg_packet.b_o_s = if is_beginning { 1 } else { 0 };
    ogg_packet.packet = packet[offset..].as_mut_ptr();
    ogg_packet.packetno = 0;
    ogg_packet.granulepos = 0;

    std::mem::forget(packet);
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_create(
    _: JNIEnv,
    _: JClass
) -> jlong {
    debug!("(vorbis) create");

    to_ptr!(VorbisState::new()) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_initialise(
    env: JNIEnv,
    _: JClass,
    instance: jlong,
    id_direct_buffer: JByteBuffer,
    id_offset: jint,
    id_length: jint,
    setup_direct_buffer: JByteBuffer,
    setup_offset: jint,
    setup_length: jint,
) -> jint {
    debug!("(vorbis) initialise, instance: {}", instance);

    let state = VorbisState::from_ptr(instance as VorbisStateHandle);

    /* dummy comment instance, needs non-null vendor, otherwise headerin will reject setup (codebook) packet. */
    let mut comment = std::mem::zeroed();
    vorbis_comment_init(&mut comment);
    comment.vendor = &mut 0;

    /* pass in identification header packet */
    let mut packet: ogg_packet = std::mem::zeroed();
    build_ogg_packet(env, &mut packet, id_direct_buffer, id_offset as usize, id_length as i64, true);

    let mut error = vorbis_synthesis_headerin(state.info_ptr, &mut comment, &mut packet);
    if error != 0 {
        return error | 0x01000000;
    };

    build_ogg_packet(env, &mut packet, setup_direct_buffer, setup_offset as usize, setup_length as i64, false);

    error = vorbis_synthesis_headerin(state.info_ptr, &mut comment, &mut packet);
    if error != 0 {
        return error | 0x01000000;
    };

    error = vorbis_synthesis_init(state.dsp_state.as_mut_ptr(), state.info_ptr);
    if error != 0 {
        return 0;
    };

    vorbis_block_init(state.dsp_state.as_mut_ptr(), state.block.as_mut_ptr());
    state.initialized = true;

    1
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_getChannelCount(
    _: JNIEnv,
    _: JClass,
    instance: jlong
) -> jint {
    debug!("(vorbis) getChannelCount, instance: {}", instance);

    let state = VorbisState::from_ptr(instance as VorbisStateHandle);
    state.get_info().channels
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_input(
    env: JNIEnv,
    _: JClass,
    instance: jlong,
    buffer: JByteBuffer,
    buffer_offset: jint,
    buffer_length: jint,
) -> jint {
    debug!("(vorbis) input, instance: {}, buffer_offset: {}, buffer_length: {}", instance, buffer_offset, buffer_length);

    let state = VorbisState::from_ptr(instance as VorbisStateHandle);

    /* build packet. */
    let mut packet = std::mem::zeroed();
    build_ogg_packet(env, &mut packet, buffer, buffer_offset as usize, buffer_length as i64, false);

    /* synthesize packet */
    let error = vorbis_synthesis(state.block.as_mut_ptr(), &mut packet);
    if error != 0 {
        return error
    }

    vorbis_synthesis_blockin(state.dsp_state.as_mut_ptr(), state.block.as_mut_ptr())
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_output(
    env: JNIEnv,
    _: JClass,
    instance: jlong,
    channels: jobjectArray,
    length: jint
) -> jint {
    debug!("(vorbis) output, instance: {}, length: {}", instance, length);

    let state = VorbisState::from_ptr(instance as VorbisStateHandle);
    let mut buffers = Pcm::new();

    let available = vorbis_synthesis_pcmout(state.dsp_state.as_mut_ptr(), &mut buffers.internal) as usize;
    let buffer_length = length as usize;

    let chunk = if available > buffer_length { buffer_length } else { available };
    if chunk > 0 {
        for i in 0..state.get_channel_count() {
            if let Ok(element) = env
                .get_object_array_element(channels, i)
                .and_then(|e| Ok(e.into_inner() as jfloatArray))
            {
                let pcm = buffers.pcm(i as usize, chunk);

                env
                    .set_float_array_region(element, 0, pcm)
                    .expect("Unable to write to buffers.");

                std::mem::forget(pcm);
            }
        }

        if env.exception_check().unwrap() {
            env.exception_clear().unwrap();
            return -1
        };

        vorbis_synthesis_read(state.dsp_state.as_mut_ptr(), chunk as i32);
    };

    chunk as jint
}

#[no_mangle]
pub unsafe extern "system" fn Java_com_sedmelluq_discord_lavaplayer_natives_vorbis_VorbisDecoderLibrary_destroy(
    _: JNIEnv,
    _: JClass,
    instance: jlong
) {
    debug!("(vorbis) destroy, instance: {}", instance);

    let state = VorbisState::from_ptr(instance as VorbisStateHandle);
    if state.initialized {
        vorbis_block_clear(state.block.as_mut_ptr());
        vorbis_dsp_clear(state.dsp_state.as_mut_ptr());
    }

    vorbis_info_clear(state.info_ptr);
    std::mem::drop(instance as VorbisStateHandle);
}
