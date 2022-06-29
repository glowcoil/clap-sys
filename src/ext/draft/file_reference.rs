use crate::{host::*, id::*, plugin::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_FILE_REFERENCE: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.file-reference.draft/0\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_file_reference {
    pub resource_id: clap_id,
    pub belongs_to_plugin_collection: bool,
    pub path_capacity: usize,
    pub path_size: usize,
    pub path: *mut c_char,
}

unsafe impl Send for clap_file_reference {}
unsafe impl Sync for clap_file_reference {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_file_reference {
    pub count: unsafe extern "C" fn(plugin: *const clap_plugin) -> u32,
    pub get: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        index: u32,
        file_reference: *mut clap_file_reference,
    ) -> bool,
    pub get_blake3_digest: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        resource_id: clap_id,
        digest: *mut u8,
    ) -> bool,
    pub get_file_size: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        resource_id: clap_id,
        size: *mut u64,
    ) -> bool,
    pub update_path: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        resource_id: clap_id,
        path: *const c_char,
    ) -> bool,
    pub save_resources: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_file_reference {
    pub changed: unsafe extern "C" fn(host: *const clap_host),
    pub set_dirty: unsafe extern "C" fn(host: *const clap_host, resource_id: clap_id),
}
