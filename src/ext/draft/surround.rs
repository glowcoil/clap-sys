use crate::{host::*, plugin::*};

use std::ffi::CStr;

pub const CLAP_EXT_SURROUND: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.surround.draft/1\0") };

pub const CLAP_PORT_SURROUND: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"surround\0") };

pub const CLAP_SURROUND_FL: u32 = 0;
pub const CLAP_SURROUND_FR: u32 = 1;
pub const CLAP_SURROUND_FC: u32 = 2;
pub const CLAP_SURROUND_LFE: u32 = 3;
pub const CLAP_SURROUND_BL: u32 = 4;
pub const CLAP_SURROUND_BR: u32 = 5;
pub const CLAP_SURROUND_FLC: u32 = 6;
pub const CLAP_SURROUND_FRC: u32 = 7;
pub const CLAP_SURROUND_BC: u32 = 8;
pub const CLAP_SURROUND_SL: u32 = 9;
pub const CLAP_SURROUND_SR: u32 = 10;
pub const CLAP_SURROUND_TC: u32 = 11;
pub const CLAP_SURROUND_TFL: u32 = 12;
pub const CLAP_SURROUND_TFC: u32 = 13;
pub const CLAP_SURROUND_TFR: u32 = 14;
pub const CLAP_SURROUND_TBL: u32 = 15;
pub const CLAP_SURROUND_TBC: u32 = 16;
pub const CLAP_SURROUND_TBR: u32 = 17;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_surround {
    pub get_channel_map: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            is_input: bool,
            port_index: u32,
            channel_map: *mut u8,
            channel_map_capacity: u32,
        ) -> u32,
    >,
    pub changed: Option<unsafe extern "C" fn(plugin: *const clap_plugin)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_surround {
    pub changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub get_preferred_channel_map: Option<
        unsafe extern "C" fn(
            plugin: *const clap_host,
            channel_map: *mut u8,
            channel_map_capacity: u32,
            channel_count: *mut u32,
        ),
    >,
}
