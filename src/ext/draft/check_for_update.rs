use crate::{host::*, plugin::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_CHECK_FOR_UPDATE: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.check_for_update.draft/0\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_check_for_update_info {
    pub version: *const c_char,
    pub release_date: *const c_char,
    pub url: *const c_char,
    pub is_preview: bool,
}

unsafe impl Send for clap_check_for_update_info {}
unsafe impl Sync for clap_check_for_update_info {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_check_for_update {
    pub check: Option<unsafe extern "C" fn(plugin: *const clap_plugin, include_preview: bool)>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_check_for_update {
    pub on_new_version: Option<
        unsafe extern "C" fn(
            host: *const clap_host,
            update_info: *const clap_check_for_update_info,
        ),
    >,
}
