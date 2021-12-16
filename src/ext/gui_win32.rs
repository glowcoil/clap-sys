use crate::plugin::*;

use std::ffi::c_void;
use std::os::raw::c_char;

pub const CLAP_EXT_GUI_WIN32: *const c_char = b"clap.gui-win32\0".as_ptr() as *const c_char;

pub type clap_hwnd = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_gui_win32 {
    pub attach: unsafe extern "C" fn(plugin: *const clap_plugin, window: clap_hwnd) -> bool,
}
