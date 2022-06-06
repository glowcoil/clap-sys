use crate::{hash::*, host::*, id::*, plugin::*, string_sizes::*};

use std::os::raw::c_char;

pub const CLAP_EXT_QUICK_CONTROLS: *const c_char =
    b"clap.quick-controls.draft/0\0".as_ptr() as *const c_char;

pub const CLAP_QUICK_CONTROLS_COUNT: usize = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_quick_controls_page {
    pub id: clap_id,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub keywords: [c_char; CLAP_KEYWORDS_SIZE],
    pub param_ids: [clap_id; CLAP_QUICK_CONTROLS_COUNT],
}

unsafe impl Send for clap_quick_controls_page {}
unsafe impl Sync for clap_quick_controls_page {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_quick_controls {
    pub count: unsafe extern "C" fn(plugin: *const clap_plugin) -> u32,
    pub get: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        page_index: u32,
        page: *mut clap_quick_controls_page,
    ) -> bool,
    pub select: unsafe extern "C" fn(plugin: *const clap_plugin, page_id: clap_id),
    pub get_selected: unsafe extern "C" fn(plugin: *const clap_plugin) -> clap_id,
}

pub const CLAP_QUICK_CONTROLS_PAGES_CHANGED: clap_quick_controls_changed_flags = 1 << 0;
pub const CLAP_QUICK_CONTROLS_SELECTED_PAGE_CHANGED: clap_quick_controls_changed_flags = 1 << 1;

pub type clap_quick_controls_changed_flags = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_quick_controls {
    pub changed:
        unsafe extern "C" fn(host: *const clap_host, flags: clap_quick_controls_changed_flags),
}
