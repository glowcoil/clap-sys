use crate::{hash::*, host::*, id::*, plugin::*, string_sizes::*};

use std::os::raw::c_char;

pub const CLAP_EXT_FILE_REFERENCE: *const c_char =
    b"clap.file-reference.draft/0\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_file_reference {
    pub resource_id: clap_id,
    pub path: [c_char; CLAP_PATH_SIZE],
    pub belongs_to_plugin_collection: bool,
}

unsafe impl Send for clap_file_reference {}
unsafe impl Sync for clap_file_reference {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_file_reference {
    pub count: unsafe extern "C" fn(plugin: *const clap_plugin) -> u32,
    pub get: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        index: u32,
        file_reference: *mut clap_file_reference,
    ) -> bool,
    pub get_hash: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        resource_id: clap_id,
        hash: clap_hash,
        digest: *mut u8,
        digest_size: u32,
    ) -> bool,
    pub update_path: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        resource_id: clap_id,
        path: *const c_char,
    ) -> bool,
    pub save_resources: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_file_reference {
    pub changed: unsafe extern "C" fn(host: *const clap_host),
    pub set_dirty: unsafe extern "C" fn(host: *const clap_host, resource_id: clap_id),
}
