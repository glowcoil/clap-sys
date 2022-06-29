use crate::{host::*, plugin::*};

use std::ffi::CStr;

pub const CLAP_EXT_THREAD_POOL: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.thread-pool\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_thread_pool {
    pub exec: unsafe extern "C" fn(plugin: *const clap_plugin, task_index: u32),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_thread_pool {
    pub request_exec: unsafe extern "C" fn(host: *const clap_host, num_tasks: u32) -> bool,
}
