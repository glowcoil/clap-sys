use crate::plugin::*;

use std::os::raw::c_char;

pub const CLAP_EXT_GUI_FREE_STANDING: *const c_char =
    b"clap.gui-free-standing\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_gui_free_standing {
    pub open: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
}
