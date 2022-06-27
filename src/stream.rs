use std::ffi::c_void;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_istream {
    pub ctx: *mut c_void,
    pub read:
        unsafe extern "C" fn(stream: *const clap_istream, buffer: *mut c_void, size: u64) -> i64,
}

unsafe impl Send for clap_istream {}
unsafe impl Sync for clap_istream {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_ostream {
    pub ctx: *mut c_void,
    pub write:
        unsafe extern "C" fn(stream: *const clap_ostream, buffer: *const c_void, size: u64) -> i64,
}

unsafe impl Send for clap_ostream {}
unsafe impl Sync for clap_ostream {}
