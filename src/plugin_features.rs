use std::os::raw::c_char;

pub const CLAP_PLUGIN_FEATURE_INSTRUMENT: *const c_char = "instrument\0".as_ptr() as *const c_char;

pub const CLAP_PLUGIN_FEATURE_AUDIO_EFFECT: *const c_char =
    "audio-effect\0".as_ptr() as *const c_char;

pub const CLAP_PLUGIN_FEATURE_NOTE_EFFECT: *const c_char =
    "note-effect\0".as_ptr() as *const c_char;

pub const CLAP_PLUGIN_FEATURE_ANALYZER: *const c_char = "analyzer\0".as_ptr() as *const c_char;

pub const CLAP_PLUGIN_FEATURE_FILTER: *const c_char = "filter\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_PHASER: *const c_char = "phaser\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_EQUALIZER: *const c_char = "equalizer\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_DEESSER: *const c_char = "de-esser\0".as_ptr() as *const c_char;

pub const CLAP_PLUGIN_FEATURE_DISTORTION: *const c_char = "distortion\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_TRANSIENT_SHAPER: *const c_char =
    "transient-shaper\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_COMPRESSOR: *const c_char = "compressor\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_LIMITER: *const c_char = "limiter\0".as_ptr() as *const c_char;

pub const CLAP_PLUGIN_FEATURE_FLANGER: *const c_char = "flanger\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_CHORUS: *const c_char = "chorus\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_DELAY: *const c_char = "delay\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_REVERB: *const c_char = "reverb\0".as_ptr() as *const c_char;

pub const CLAP_PLUGIN_FEATURE_GLITCH: *const c_char = "glitch\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_UTILITY: *const c_char = "utility\0".as_ptr() as *const c_char;

pub const CLAP_PLUGIN_FEATURE_MONO: *const c_char = "mono\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_STEREO: *const c_char = "stereo\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_SURROUND: *const c_char = "surround\0".as_ptr() as *const c_char;
pub const CLAP_PLUGIN_FEATURE_AMBISONIC: *const c_char = "ambisonic\0".as_ptr() as *const c_char;

pub const CLAP_PLUGIN_FEATURE_WIN32_DPI_AWARE: *const c_char =
    "win32-dpi-aware\0".as_ptr() as *const c_char;
