use crate::host::*;

use std::ffi::CStr;

pub const CLAP_EXT_THREAD_CHECK: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.thread-check\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_thread_check {
    pub is_main_thread: Option<unsafe extern "C" fn(host: *const clap_host) -> bool>,
    pub is_audio_thread: Option<unsafe extern "C" fn(host: *const clap_host) -> bool>,
}
