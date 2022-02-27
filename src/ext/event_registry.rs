use crate::host::*;

use std::os::raw::c_char;

pub const CLAP_EXT_EVENT_REGISTRY: *const c_char =
    b"clap.event-registry\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_event_registry {
    pub query: unsafe extern "C" fn(
        host: *const clap_host,
        space_name: *const c_char,
        space_id: *mut u16,
    ) -> bool,
}
