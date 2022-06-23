use crate::plugin::*;

use std::os::raw::c_char;

pub const CLAP_EXT_PRESET_LOAD: *const c_char =
    b"clap.preset-load.draft/0\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_preset_load {
    pub from_file: unsafe extern "C" fn(plugin: *const clap_plugin, path: *const c_char) -> bool,
}
