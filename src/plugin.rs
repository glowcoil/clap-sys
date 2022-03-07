use crate::{process::*, version::*};

use std::ffi::c_void;
use std::os::raw::c_char;

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
    pub features: *const *const c_char,
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
    pub reset: unsafe extern "C" fn(plugin: *const clap_plugin),
    pub process: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        process: *const clap_process,
    ) -> clap_process_status,
    pub get_extension:
        unsafe extern "C" fn(plugin: *const clap_plugin, id: *const c_char) -> *const c_void,
    pub on_main_thread: unsafe extern "C" fn(plugin: *const clap_plugin),
}

unsafe impl Send for clap_plugin {}
unsafe impl Sync for clap_plugin {}
