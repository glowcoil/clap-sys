use crate::host::*;

use std::os::raw::c_char;

pub const CLAP_EXT_THREAD_CHECK: *const c_char = b"clap.thread-check\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_thread_check {
    pub is_main_thread: unsafe extern "C" fn(host: *const clap_host) -> bool,
    pub is_audio_thread: unsafe extern "C" fn(host: *const clap_host) -> bool,
}
