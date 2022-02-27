use crate::version::*;

use std::ffi::c_void;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_entry {
    pub clap_version: clap_version,
    pub init: unsafe extern "C" fn(plugin_path: *const c_char) -> bool,
    pub deinit: unsafe extern "C" fn(),
    pub get_factory: unsafe extern "C" fn(factory_id: *const c_char) -> *const c_void,
}
