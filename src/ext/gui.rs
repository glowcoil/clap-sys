use crate::{host::*, plugin::*};

use std::ffi::c_void;
use std::os::raw::{c_char, c_ulong};

pub const CLAP_EXT_GUI: *const c_char = b"clap.gui\0".as_ptr() as *const c_char;

pub const CLAP_WINDOW_API_WIN32: *const c_char = b"win32\0".as_ptr() as *const c_char;
pub const CLAP_WINDOW_API_COCOA: *const c_char = b"cocoa\0".as_ptr() as *const c_char;
pub const CLAP_WINDOW_API_X11: *const c_char = b"x11\0".as_ptr() as *const c_char;
pub const CLAP_WINDOW_API_WAYLAND: *const c_char = b"wayland\0".as_ptr() as *const c_char;

pub type clap_hwnd = *mut c_void;
pub type clap_nsview = *mut c_void;
pub type clap_xwnd = c_ulong;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_window {
    pub api: *const c_char,
    pub specific: clap_window_handle,
}

/// Defined as an anonymous union in [`clap_window` ]in the C-version.
#[repr(C)]
#[derive(Copy, Clone)]
pub union clap_window_handle {
    pub cocoa: clap_nsview,
    pub x11: clap_xwnd,
    pub win32: clap_hwnd,
    pub ptr: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_gui {
    pub is_api_supported: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        api: *const c_char,
        is_floating: bool,
    ) -> bool,
    pub create: unsafe extern "C" fn(
        plugin: *const clap_plugin,
        api: *const c_char,
        is_floating: bool,
    ) -> bool,
    pub destroy: unsafe extern "C" fn(plugin: *const clap_plugin),
    pub set_scale: unsafe extern "C" fn(plugin: *const clap_plugin, scale: f64) -> bool,
    pub get_size:
        unsafe extern "C" fn(plugin: *const clap_plugin, width: *mut u32, height: *mut u32) -> bool,
    pub can_resize: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
    pub adjust_size:
        unsafe extern "C" fn(plugin: *const clap_plugin, width: *mut u32, height: *mut u32) -> bool,
    pub set_size: unsafe extern "C" fn(plugin: *const clap_plugin, width: u32, height: u32) -> bool,
    pub set_parent:
        unsafe extern "C" fn(plugin: *const clap_plugin, window: *const clap_window) -> bool,
    pub set_transient:
        unsafe extern "C" fn(plugin: *const clap_plugin, window: *const clap_window) -> bool,
    pub suggest_title: unsafe extern "C" fn(plugin: *const clap_plugin, title: *const c_char),
    pub show: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
    pub hide: unsafe extern "C" fn(plugin: *const clap_plugin) -> bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_gui {
    pub resize: unsafe extern "C" fn(host: *const clap_host, width: u32, height: u32) -> bool,
    pub request_show: unsafe extern "C" fn(host: *const clap_host),
    pub request_hide: unsafe extern "C" fn(host: *const clap_host),
    pub closed: unsafe extern "C" fn(host: *const clap_host, was_destroyed: bool),
}
