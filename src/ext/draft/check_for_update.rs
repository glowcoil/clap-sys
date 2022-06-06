use crate::host::*;

use std::os::raw::c_char;

pub const CLAP_EXT_CHECK_FOR_UPDATE: *const c_char =
    b"clap.check_for_update.draft/0\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_check_for_update_info {
    pub version: *const c_char,
    pub release_date: *const c_char,
    pub url: *const c_char,
    pub is_stable: bool,
}

unsafe impl Send for clap_check_for_update_info {}
unsafe impl Sync for clap_check_for_update_info {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_check_for_update {
    // FIXME: This `clap_host` should probably be a `clap_plugin`, see
    //        https://github.com/free-audio/clap/issues/79
    pub check: unsafe extern "C" fn(plugin: *const clap_host, include_beta: bool),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_check_for_update {
    pub on_new_version: unsafe extern "C" fn(
        host: *const clap_host,
        update_info: *const clap_check_for_update_info,
    ),
}
