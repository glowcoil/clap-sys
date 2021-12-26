use crate::{host::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_EXT_FD_SUPPORT: *const c_char = b"clap.fd-support\0".as_ptr() as *const c_char;

#[cfg(target_os = "windows")]
pub type clap_fd = *mut ::core::ffi::c_void;
#[cfg(not(target_os = "windows"))]
pub type clap_fd = i32;

pub const CLAP_FD_READ: clap_fd_flags = 1 << 0;
pub const CLAP_FD_WRITE: clap_fd_flags = 1 << 1;
pub const CLAP_FD_ERROR: clap_fd_flags = 1 << 2;

pub type clap_fd_flags = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_fd_support {
    pub on_fd: unsafe extern "C" fn(plugin: *const clap_plugin, fd: clap_fd, flags: clap_fd_flags),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_host_fd_support {
    pub register_fd:
        unsafe extern "C" fn(host: *const clap_host, fd: clap_fd, flags: clap_fd_flags) -> bool,
    pub modify_fd:
        unsafe extern "C" fn(host: *const clap_host, fd: clap_fd, flags: clap_fd_flags) -> bool,
    pub unregister_fd: unsafe extern "C" fn(host: *const clap_host, fd: clap_fd) -> bool,
}
