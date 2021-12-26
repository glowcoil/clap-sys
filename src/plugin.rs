use crate::{host::*, process::*, version::*};

use std::ffi::c_void;
use std::os::raw::c_char;

pub const CLAP_PLUGIN_INSTRUMENT: clap_plugin_type = 1 << 0;
pub const CLAP_PLUGIN_AUDIO_EFFECT: clap_plugin_type = 1 << 1;
pub const CLAP_PLUGIN_EVENT_EFFECT: clap_plugin_type = 1 << 2;
pub const CLAP_PLUGIN_ANALYZER: clap_plugin_type = 1 << 3;

pub type clap_plugin_type = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_descriptor {
    pub clap_version: clap_version,
    pub id: *const c_char,
    pub name: *const c_char,
    pub vendor: *const c_char,
    pub url: *const c_char,
    pub manual_url: *const c_char,
    pub support_url: *const c_char,
    pub version: *const c_char,
    pub description: *const c_char,
    pub keywords: *const c_char,
    pub plugin_type: u64,
}

unsafe impl Send for clap_plugin_descriptor {}
unsafe impl Sync for clap_plugin_descriptor {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin {
    pub desc: *const clap_plugin_descriptor,
    pub plugin_data: *mut c_void,
    pub init: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
    pub destroy: unsafe extern "C" fn(plugin: *const clap_plugin),
    pub activate: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        sample_rate: f64,
        min_frames_count: u32,
        max_frames_count: u32,
    ) -> bool,
    pub deactivate: unsafe extern "C" fn(plugin: *const clap_plugin),
    pub start_processing: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
    pub stop_processing: unsafe extern "C" fn(plugin: *const clap_plugin),
    pub process: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        process: *const clap_process,
    ) -> clap_process_status,
    pub get_extension:
        unsafe extern "C" fn(plugin: *const clap_plugin, id: *const c_char) -> *const c_void,
    pub on_main_thread: unsafe extern "C" fn(plugin: *const clap_plugin),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_invalidation_source {
    pub directory: *const c_char,
    pub filename_glob: *const c_char,
    pub recursive_scan: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_entry {
    pub clap_version: clap_version,
    pub init: unsafe extern "C" fn(plugin_path: *const c_char),
    pub deinit: unsafe extern "C" fn(),
    pub get_plugin_count: unsafe extern "C" fn() -> u32,
    pub get_plugin_descriptor: unsafe extern "C" fn(index: u32) -> *const clap_plugin_descriptor,
    pub create_plugin: unsafe extern "C" fn(
        host: *const clap_host,
        plugin_id: *const c_char,
    ) -> *const clap_plugin,
    pub get_invalidation_source_count: unsafe extern "C" fn() -> u32,
    pub get_invalidation_source:
        unsafe extern "C" fn(index: u32) -> *const clap_plugin_invalidation_source,
    pub refresh: unsafe extern "C" fn(),
}
