use crate::{host::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_CV: *const c_char = b"clap.cv.draft/0\0".as_ptr() as *const c_char;

pub const CLAP_PORT_CV: *const c_char = b"cv\0".as_ptr() as *const c_char;

pub const CLAP_CV_VALUE: u32 = 0;
pub const CLAP_CV_GATE: u32 = 1;
pub const CLAP_CV_PITCH: u32 = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_cv {
    pub get_channel_type: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        is_input: bool,
        port_index: u32,
        channel_index: u32,
        channel_type: *mut u32,
    ) -> bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_cv {
    pub changed: unsafe extern "C" fn(host: *const clap_host),
}
