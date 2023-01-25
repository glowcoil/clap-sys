use std::ffi::CStr;

pub const CLAP_PLUGIN_FEATURE_INSTRUMENT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"instrument\0") };

pub const CLAP_PLUGIN_FEATURE_AUDIO_EFFECT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"audio-effect\0") };

pub const CLAP_PLUGIN_FEATURE_NOTE_EFFECT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"note-effect\0") };

pub const CLAP_PLUGIN_FEATURE_NOTE_DETECTOR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"note-detector\0") };

pub const CLAP_PLUGIN_FEATURE_ANALYZER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"analyzer\0") };

pub const CLAP_PLUGIN_FEATURE_SYNTHESIZER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"synthesizer\0") };
pub const CLAP_PLUGIN_FEATURE_SAMPLER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"sampler\0") };
pub const CLAP_PLUGIN_FEATURE_DRUM: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"drum\0") };
pub const CLAP_PLUGIN_FEATURE_DRUM_MACHINE: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"drum-machine\0") };

pub const CLAP_PLUGIN_FEATURE_FILTER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"filter\0") };
pub const CLAP_PLUGIN_FEATURE_PHASER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"phaser\0") };
pub const CLAP_PLUGIN_FEATURE_EQUALIZER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"equalizer\0") };
pub const CLAP_PLUGIN_FEATURE_DEESSER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"de-esser\0") };
pub const CLAP_PLUGIN_FEATURE_PHASE_VOCODER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"phase-vocoder\0") };
pub const CLAP_PLUGIN_FEATURE_GRANULAR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"granular\0") };
pub const CLAP_PLUGIN_FEATURE_FREQUENCY_SHIFTER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"frequency-shifter\0") };
pub const CLAP_PLUGIN_FEATURE_PITCH_SHIFTER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"pitch-shifter\0") };

pub const CLAP_PLUGIN_FEATURE_DISTORTION: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"distortion\0") };
pub const CLAP_PLUGIN_FEATURE_TRANSIENT_SHAPER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"transient-shaper\0") };
pub const CLAP_PLUGIN_FEATURE_COMPRESSOR: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"compressor\0") };
pub const CLAP_PLUGIN_FEATURE_LIMITER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"limiter\0") };

pub const CLAP_PLUGIN_FEATURE_FLANGER: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"flanger\0") };
pub const CLAP_PLUGIN_FEATURE_CHORUS: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"chorus\0") };
pub const CLAP_PLUGIN_FEATURE_DELAY: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"delay\0") };
pub const CLAP_PLUGIN_FEATURE_REVERB: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"reverb\0") };

pub const CLAP_PLUGIN_FEATURE_TREMOLO: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"tremolo\0") };
pub const CLAP_PLUGIN_FEATURE_GLITCH: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"glitch\0") };

pub const CLAP_PLUGIN_FEATURE_UTILITY: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"utility\0") };
pub const CLAP_PLUGIN_FEATURE_PITCH_CORRECTION: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"pitch-correction\0") };
pub const CLAP_PLUGIN_FEATURE_RESTORATION: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"restoration\0") };

pub const CLAP_PLUGIN_FEATURE_MULTI_EFFECTS: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"multi-effects\0") };

pub const CLAP_PLUGIN_FEATURE_MIXING: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mixing\0") };
pub const CLAP_PLUGIN_FEATURE_MASTERING: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mastering\0") };

pub const CLAP_PLUGIN_FEATURE_MONO: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"mono\0") };
pub const CLAP_PLUGIN_FEATURE_STEREO: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"stereo\0") };
pub const CLAP_PLUGIN_FEATURE_SURROUND: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"surround\0") };
pub const CLAP_PLUGIN_FEATURE_AMBISONIC: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"ambisonic\0") };
