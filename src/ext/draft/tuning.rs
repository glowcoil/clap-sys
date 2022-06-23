use crate::{events::*, host::*, id::*, plugin::*, string_sizes::*};

use std::os::raw::c_char;

pub const CLAP_EXT_TUNING: *const c_char = b"clap.tuning.draft/2\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_event_tuning {
    pub header: clap_event_header,
    pub port_index: i16,
    pub channel: i16,
    pub tunning_id: clap_id,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_tuning_info {
    pub tuning_id: clap_id,
    pub name: [c_char; CLAP_NAME_SIZE],
    pub is_dynamic: bool,
}

unsafe impl Send for clap_tuning_info {}
unsafe impl Sync for clap_tuning_info {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_tuning_t {
    pub changed: unsafe extern "C" fn(plugin: *const clap_plugin),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_tuning {
    pub get_relative: unsafe extern "C" fn(
        host: *const clap_host,
        tuning_id: clap_id,
        channel: i32,
        key: i32,
        sample_offset: u32,
    ) -> f64,
    pub should_play: unsafe extern "C" fn(
        host: *const clap_host,
        tuning_id: clap_id,
        channel: i32,
        key: i32,
    ) -> bool,
    pub get_tuning_count: unsafe extern "C" fn(host: *const clap_host) -> u32,
    pub get_info: unsafe extern "C" fn(
        host: *const clap_host,
        tuning_index: u32,
        info: *mut clap_tuning_info,
    ) -> bool,
}
