use crate::{host::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_THREAD_POOL: *const c_char = b"clap.thread-pool\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_thread_pool {
    pub exec: unsafe extern "C" fn(plugin: *const clap_plugin, task_index: u32),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_thread_pool {
    pub request_exec: unsafe extern "C" fn(host: *const clap_host, num_tasks: u32) -> bool,
}
