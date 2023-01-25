use crate::{host::*, plugin::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_PRESET_LOAD: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.preset-load.draft/1\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_preset_load {
    pub from_uri: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            uri: *const c_char,
            load_key: *const c_char,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_preset_load {
    pub on_error: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            uri: *const c_char,
            os_error: i32,
            msg: *const c_char,
        ) -> bool,
    >,
}
