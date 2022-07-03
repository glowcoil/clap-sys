use crate::{host::*, plugin::*};

use std::ffi::CStr;

pub const CLAP_EXT_POSIX_FD_SUPPORT: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.posix-fd-support\0") };

pub const CLAP_POSIX_FD_READ: clap_posix_fd_flags = 1 << 0;
pub const CLAP_POSIX_FD_WRITE: clap_posix_fd_flags = 1 << 1;
pub const CLAP_POSIX_FD_ERROR: clap_posix_fd_flags = 1 << 2;

pub type clap_posix_fd_flags = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_posix_fd_support {
    pub on_fd: Option<
        unsafe extern "C" fn(plugin: *const clap_plugin, fd: i32, flags: clap_posix_fd_flags),
    >,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_host_posix_fd_support {
    pub register_fd: Option<
        unsafe extern "C" fn(host: *const clap_host, fd: i32, flags: clap_posix_fd_flags) -> bool,
    >,
    pub modify_fd: Option<
        unsafe extern "C" fn(host: *const clap_host, fd: i32, flags: clap_posix_fd_flags) -> bool,
    >,
    pub unregister_fd: Option<unsafe extern "C" fn(host: *const clap_host, fd: i32) -> bool>,
}
