#[macro_use]
mod macros;
mod util;

pub mod connector {
    mod aac;
    mod opus;
    mod samplerate;
    mod vorbis;
    mod mp3;
    mod debug;
}

/* jni stuff basically. */
pub use connector::*;
