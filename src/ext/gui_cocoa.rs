use crate::plugin::*;

use std::ffi::c_void;
use std::os::raw::c_char;

pub const CLAP_EXT_GUI_COCOA: *const c_char = b"clap.gui-cocoa\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_gui_cocoa {
    pub attach: unsafe extern "C" fn(plugin: *const clap_plugin, nsView: *mut c_void) -> bool,
}
