use crate::{host::*, id::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_TIMER_SUPPORT: *const c_char = b"clap.timer-support\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_timer_support {
    pub on_timer: unsafe extern "C" fn(plugin: *const clap_plugin, timer_id: clap_id),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_timer_support {
    pub register_timer: unsafe extern "C" fn(
        host: *const clap_host,
        period_ms: u32,
        timer_id: *mut clap_id,
    ) -> bool,
    pub unregister_timer: unsafe extern "C" fn(host: *const clap_host, timer_id: clap_id) -> bool,
}
