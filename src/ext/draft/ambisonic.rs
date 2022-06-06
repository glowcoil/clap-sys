use crate::{host::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_AMBISONIC: *const c_char = b"clap.ambisonic.draft/0\0".as_ptr() as *const c_char;

pub const CLAP_PORT_AMBISONIC: *const c_char = b"ambisonic\0".as_ptr() as *const c_char;

pub const CLAP_AMBISONIC_FUMA: u32 = 0;
pub const CLAP_AMBISONIC_ACN: u32 = 1;

pub const CLAP_AMBISONIC_NORMALIZATION_MAXN: u32 = 0;
pub const CLAP_AMBISONIC_NORMALIZATION_SN3D: u32 = 1;
pub const CLAP_AMBISONIC_NORMALIZATION_N3D: u32 = 2;
pub const CLAP_AMBISONIC_NORMALIZATION_SN2D: u32 = 3;
pub const CLAP_AMBISONIC_NORMALIZATION_N2D: u32 = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_ambisonic_info {
    pub ordering: u32,
    pub normalization: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_ambisonic {
    pub get_info: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        is_input: bool,
        port_index: u32,
        info: *mut clap_ambisonic_info,
    ) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_ambisonic {
    pub changed: unsafe extern "C" fn(host: *const clap_host),
}
