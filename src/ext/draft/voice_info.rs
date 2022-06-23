use crate::{host::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_VOICE_INFO: *const c_char =
    b"clap.voice-info.draft/0\0".as_ptr() as *const c_char;

pub const CLAP_VOICE_INFO_SUPPORTS_OVERLAPPING_NOTES: u64 = 1 << 0;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_voice_info {
    pub voice_count: u32,
    pub voice_capacity: u32,
    pub flags: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_voice_info {
    pub get: unsafe extern "C" fn(plugin: *const clap_plugin, info: *mut clap_voice_info) -> bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_voice_info {
    pub changed: unsafe extern "C" fn(host: *const clap_host),
}
