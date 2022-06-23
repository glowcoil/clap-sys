use crate::{host::*, id::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_MIDI_MAPPINGS: *const c_char =
    b"clap.midi-mappings.draft/0\0".as_ptr() as *const c_char;

pub const CLAP_MIDI_MAPPING_CC7: clap_midi_mapping_type = 0;
pub const CLAP_MIDI_MAPPING_CC14: clap_midi_mapping_type = 1;
pub const CLAP_MIDI_MAPPING_RPN: clap_midi_mapping_type = 2;
pub const CLAP_MIDI_MAPPING_NRPN: clap_midi_mapping_type = 3;

// Not used or part of the enum in the current draft implementation, but we'll assume this was the
// intention. Actually, these constants are also never reference anywhere...
pub type clap_midi_mapping_type = i32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_midi_mapping {
    pub channel: i32,
    pub number: i32,
    pub param_id: clap_id,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_midi_mappings {
    pub count: unsafe extern "C" fn(plugin: *const clap_plugin) -> u32,
    pub get: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        index: u32,
        mapping: *mut clap_midi_mapping,
    ) -> bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_midi_mappings {
    pub changed: unsafe extern "C" fn(host: *const clap_host),
}
