use crate::host::*;

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_EVENT_REGISTRY: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.event-registry\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_event_registry {
    pub query: unsafe extern "C" fn(
        host: *const clap_host,
        space_name: *const c_char,
        space_id: *mut u16,
    ) -> bool,
}
