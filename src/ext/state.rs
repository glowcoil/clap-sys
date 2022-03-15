use crate::{host::*, plugin::*, stream::*};

use std::os::raw::c_char;

pub const CLAP_EXT_STATE: *const c_char = b"clap.state\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_state {
    pub save: unsafe extern "C" fn(plugin: *const clap_plugin, stream: *const clap_ostream) -> bool,
    pub load: unsafe extern "C" fn(plugin: *const clap_plugin, stream: *const clap_istream) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_state {
    pub mark_dirty: unsafe extern "C" fn(host: *const clap_host),
}
