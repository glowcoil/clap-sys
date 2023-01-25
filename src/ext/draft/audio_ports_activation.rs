use crate::plugin::*;

use std::ffi::CStr;

pub const CLAP_EXT_AUDIO_PORTS_ACTIVATION: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.audio-ports-activation/draft-1\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_audio_ports_activation {
    pub can_activate_while_processing:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> bool>,
    pub set_active: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            is_input: bool,
            port_index: u32,
            is_active: bool,
        ) -> bool,
    >,
}
