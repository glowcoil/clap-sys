use crate::{host::*, id::*, plugin::*, string_sizes::*};

use std::os::raw::c_char;

pub const CLAP_EXT_NOTE_PORTS: *const c_char = b"clap.note-ports\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_note_port_info {
    pub id: clap_id,
    pub name: [c_char; CLAP_NAME_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_note_ports {
    pub count: unsafe extern "C" fn(plugin: *const clap_plugin, is_input: bool) -> u32,
    pub get: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        index: u32,
        is_input: bool,
        info: *mut clap_note_port_info,
    ) -> bool,
}

pub const CLAP_NOTE_PORTS_RESCAN_ALL: u32 = 1 << 0;
pub const CLAP_NOTE_PORTS_RESCAN_NAMES: u32 = 1 << 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_note_ports {
    pub rescan: unsafe extern "C" fn(host: *const clap_host, flags: u32),
}
