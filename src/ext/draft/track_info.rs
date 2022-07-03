use crate::{color::*, host::*, id::*, plugin::*, string_sizes::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_TRACK_INFO: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.track-info.draft/0\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_track_info {
    pub id: clap_id,
    pub index: i32,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub path: [c_char; CLAP_PATH_SIZE],
    pub channel_count: i32,
    pub audio_port_type: *const c_char,
    pub color: clap_color,
    pub is_return_track: bool,
}

unsafe impl Send for clap_track_info {}
unsafe impl Sync for clap_track_info {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_track_info {
    pub changed: Option<unsafe extern "C" fn(plugin: *const clap_plugin)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_track_info {
    pub get:
        Option<unsafe extern "C" fn(host: *const clap_host, info: *mut clap_track_info) -> bool>,
}
