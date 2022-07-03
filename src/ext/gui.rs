use crate::{host::*, plugin::*};

use std::ffi::{c_void, CStr};
use std::fmt::Debug;
use std::os::raw::{c_char, c_ulong};

pub const CLAP_EXT_GUI: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.gui\0") };

pub const CLAP_WINDOW_API_WIN32: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"win32\0") };
pub const CLAP_WINDOW_API_COCOA: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"cocoa\0") };
pub const CLAP_WINDOW_API_X11: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"x11\0") };
pub const CLAP_WINDOW_API_WAYLAND: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"wayland\0") };

pub type clap_hwnd = *mut c_void;
pub type clap_nsview = *mut c_void;
pub type clap_xwnd = c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_window {
    pub api: *const c_char,
    pub specific: clap_window_handle,
}

unsafe impl Send for clap_window {}
unsafe impl Sync for clap_window {}

/// Defined as an anonymous union in [`clap_window`] in the C-version.
#[repr(C)]
#[derive(Copy, Clone)]
pub union clap_window_handle {
    pub cocoa: clap_nsview,
    pub x11: clap_xwnd,
    pub win32: clap_hwnd,
    pub ptr: *mut c_void,
}

unsafe impl Send for clap_window_handle {}
unsafe impl Sync for clap_window_handle {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_gui_resize_hints {
    pub can_resize_horizontally: bool,
    pub can_resize_vertically: bool,
    pub preserve_aspect_ratio: bool,
    pub aspect_ratio_width: u32,
    pub aspect_ratio_height: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_gui {
    pub is_api_supported: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            api: *const c_char,
            is_floating: bool,
        ) -> bool,
    >,
    pub get_preferred_api: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            api: *mut *const c_char,
            is_floating: *mut bool,
        ) -> bool,
    >,
    pub create: Option<
        unsafe extern "C" fn(
            plugin: *const clap_plugin,
            api: *const c_char,
            is_floating: bool,
        ) -> bool,
    >,
    pub destroy: Option<unsafe extern "C" fn(plugin: *const clap_plugin)>,
    pub set_scale: Option<unsafe extern "C" fn(plugin: *const clap_plugin, scale: f64) -> bool>,
    pub get_size: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, width: *mut u32, height: *mut u32) -> bool,
    >,
    pub can_resize: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> bool>,
    pub get_resize_hints: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, hints: *mut clap_gui_resize_hints) -> bool,
    >,
    pub adjust_size: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, width: *mut u32, height: *mut u32) -> bool,
    >,
    pub set_size:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin, width: u32, height: u32) -> bool>,
    pub set_parent: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, window: *const clap_window) -> bool,
    >,
    pub set_transient: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, window: *const clap_window) -> bool,
    >,
    pub suggest_title:
        Option<unsafe extern "C" fn(plugin: *const clap_plugin, title: *const c_char)>,
    pub show: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> bool>,
    pub hide: Option<unsafe extern "C" fn(plugin: *const clap_plugin) -> bool>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_gui {
    pub resize_hints_changed: Option<unsafe extern "C" fn(host: *const clap_host)>,
    pub request_resize:
        Option<unsafe extern "C" fn(host: *const clap_host, width: u32, height: u32) -> bool>,
    pub request_show: Option<unsafe extern "C" fn(host: *const clap_host) -> bool>,
    pub request_hide: Option<unsafe extern "C" fn(host: *const clap_host) -> bool>,
    pub closed: Option<unsafe extern "C" fn(host: *const clap_host, was_destroyed: bool)>,
}

impl Debug for clap_window_handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // We can't know which variant this actually is without supposed to be without checking the
        // `api` field in `clap_window`, but that cannot be done safely
        f.debug_struct("clap_window_handle").finish_non_exhaustive()
    }
}
