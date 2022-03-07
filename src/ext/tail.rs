use crate::{host::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_TAIL: *const c_char = b"clap.tail\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_tail {
    pub get: unsafe extern "C" fn(plugin: *const clap_plugin) -> u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_tail {
    pub changed: unsafe extern "C" fn(host: *const clap_host),
}
