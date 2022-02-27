use crate::{events::*, host::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_EVENT_FILTER: *const c_char = b"clap.event-filter\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_event_filter {
    pub accepts: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        space_id: u16,
        event_type: clap_event_type,
    ) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_event_filter {
    pub changed: unsafe extern "C" fn(host: *const clap_host),
}
