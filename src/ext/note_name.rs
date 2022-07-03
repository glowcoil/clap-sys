use crate::{host::*, plugin::*, string_sizes::*};

use std::ffi::CStr;
use std::os::raw::c_char;

pub const CLAP_EXT_NOTE_NAME: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.note-name\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_note_name {
    pub name: [c_char; CLAP_NAME_SIZE],
    pub port: i16,
    pub key: i16,
    pub channel: i16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_note_name {
    pub count: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> u32>,
    pub get: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            index: u32,
            note_name: *mut clap_note_name,
        ) -> bool,
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_note_name {
    pub changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
}
