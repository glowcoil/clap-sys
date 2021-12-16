use crate::{chmap::*, host::*, id::*, plugin::*, string_sizes::*};

use std::os::raw::c_char;

pub const CLAP_EXT_AUDIO_PORTS_CONFIG: *const c_char =
    b"clap.audio-ports-config\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_audio_ports_config {
    pub id: clap_id,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub input_channel_count: u32,
    pub input_channel_map: clap_chmap,
    pub output_channel_count: u32,
    pub output_channel_map: clap_chmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_audio_ports_config {
    pub count: unsafe extern "C" fn(plugin: *const clap_plugin),
    pub get: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        index: u32,
        config: *mut clap_audio_ports_config,
    ),
    pub select: unsafe extern "C" fn(plugin: *const clap_plugin, config_id: clap_id),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_audio_ports_config {
    pub rescan: unsafe extern "C" fn(host: *const clap_host),
}
