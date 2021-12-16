use crate::plugin::*;

use std::ffi::c_void;
use std::os::raw::c_char;

pub const CLAP_EXT_GUI_X11: *const c_char = b"clap.gui-x11\0".as_ptr() as *const c_char;

pub type clap_hwnd = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_gui_x11 {
    pub attach: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        display_name: *const c_char,
        window: u64,
    ) -> bool,
}
