use crate::{host::*, id::*, plugin::*, string_sizes::*};

use std::os::raw::c_char;

pub const CLAP_EXT_AUDIO_PORTS: *const c_char = b"clap.audio-ports\0".as_ptr() as *const c_char;

pub const CLAP_PORT_MONO: *const c_char = b"mono\0".as_ptr() as *const c_char;
pub const CLAP_PORT_STEREO: *const c_char = b"stereo\0".as_ptr() as *const c_char;

pub const CLAP_AUDIO_PORT_IS_MAIN: u32 = 1 << 0;
pub const CLAP_AUDIO_PORTS_PREFERS_64BITS: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_audio_port_info {
    pub id: clap_id,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub flags: u32,
    pub channel_count: u32,
    pub port_type: *const c_char,
    pub in_place_pair: clap_id,
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
    ) -> bool,
}

pub const CLAP_AUDIO_PORTS_RESCAN_ALL: u32 = 1 << 0;
pub const CLAP_AUDIO_PORTS_RESCAN_NAMES: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_audio_ports {
    pub rescan: unsafe extern "C" fn(host: *const clap_host, flags: u32),
}
