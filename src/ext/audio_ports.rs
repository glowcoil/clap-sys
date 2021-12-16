use crate::{chmap::*, host::*, id::*, plugin::*, string_sizes::*};

use std::os::raw::c_char;

pub const CLAP_EXT_AUDIO_PORTS: *const c_char = b"clap.audio-ports\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_audio_port_info {
    pub id: clap_id,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub channel_count: u32,
    pub channel_map: clap_chmap,
    pub sample_size: u32,
    pub is_main: bool,
    pub is_cv: bool,
    pub in_place: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_audio_ports {
    pub count: unsafe extern "C" fn(plugin: *const clap_plugin, is_input: bool) -> u32,
    pub get: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        index: u32,
        is_input: bool,
        info: *mut clap_audio_port_info,
    ) -> u32,
}

pub const CLAP_AUDIO_PORTS_RESCAN_ALL: u32 = 1 << 0;
pub const CLAP_AUDIO_PORTS_RESCAN_NAMES: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_audio_ports {
    pub get_preferred_sample_size: unsafe extern "C" fn(host: *const clap_host) -> u32,
    pub rescan: unsafe extern "C" fn(host: *const clap_host, flags: u32),
}
