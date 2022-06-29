use crate::{host::*, plugin::*, stream::*};

use std::ffi::CStr;

pub const CLAP_EXT_STATE: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.state\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_state {
    pub save: unsafe extern "C" fn(plugin: *const clap_plugin, stream: *const clap_ostream) -> bool,
    pub load: unsafe extern "C" fn(plugin: *const clap_plugin, stream: *const clap_istream) -> bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_state {
    pub mark_dirty: unsafe extern "C" fn(host: *const clap_host),
}
