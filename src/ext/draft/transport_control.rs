use crate::{fixedpoint::*, host::*};

use std::os::raw::c_char;

pub const CLAP_EXT_TRANSPORT_CONTROL: *const c_char =
    b"clap.transport-control.draft/0\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_transport_control {
    pub request_start: unsafe extern "C" fn(host: *const clap_host),
    pub request_stop: unsafe extern "C" fn(host: *const clap_host),
    pub request_continue: unsafe extern "C" fn(host: *const clap_host),
    pub request_pause: unsafe extern "C" fn(host: *const clap_host),
    pub request_toggle_play: unsafe extern "C" fn(host: *const clap_host),
    pub request_jump: unsafe extern "C" fn(host: *const clap_host, position: clap_beattime),
    pub request_loop_region:
        unsafe extern "C" fn(host: *const clap_host, start: clap_beattime, duration: clap_beattime),
    pub request_toggle_loop: unsafe extern "C" fn(host: *const clap_host),
    pub request_enable_loop: unsafe extern "C" fn(host: *const clap_host, is_enabled: bool),
    pub request_record: unsafe extern "C" fn(host: *const clap_host, is_recording: bool),
    pub request_toggle_record: unsafe extern "C" fn(host: *const clap_host),
}
