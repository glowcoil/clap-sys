use crate::{host::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_GUI: *const c_char = b"clap.gui\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_gui {
    pub create: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
    pub destroy: unsafe extern "C" fn(plugin: *const clap_plugin),
    pub set_scale: unsafe extern "C" fn(plugin: *const clap_plugin, scale: f64),
    pub get_size:
        unsafe extern "C" fn(plugin: *const clap_plugin, width: *mut u32, height: *mut u32) -> bool,
    pub can_resize: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
    pub round_size:
        unsafe extern "C" fn(plugin: *const clap_plugin, width: *mut u32, height: *mut u32),
    pub set_size: unsafe extern "C" fn(plugin: *const clap_plugin, width: u32, height: u32),
    pub show: unsafe extern "C" fn(plugin: *const clap_plugin),
    pub hide: unsafe extern "C" fn(plugin: *const clap_plugin),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_gui {
    pub resize: unsafe extern "C" fn(host: *const clap_host, width: u32, height: u32) -> bool,
}
