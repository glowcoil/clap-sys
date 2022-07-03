use crate::{host::*, id::*, plugin::*, string_sizes::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_QUICK_CONTROLS: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.quick-controls.draft/0\0") };

pub const CLAP_QUICK_CONTROLS_COUNT: usize = 8;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_quick_controls_page {
    pub id: clap_id,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub param_ids: [clap_id; CLAP_QUICK_CONTROLS_COUNT],
}

unsafe impl Send for clap_quick_controls_page {}
unsafe impl Sync for clap_quick_controls_page {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_quick_controls {
    pub count: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> u32>,
    pub get: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            page_index: u32,
            page: *mut clap_quick_controls_page,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_quick_controls {
    pub changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub suggest_page: Option<unsafe extern "C" fn(host: *const clap_host, page_id: clap_id)>,
}
