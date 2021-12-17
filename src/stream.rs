use std::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_istream {
    pub ctx: *mut c_void,
    pub read:
        unsafe extern "C" fn(stream: *mut clap_istream, buffer: *mut c_void, size: u64) -> i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_ostream {
    pub ctx: *mut c_void,
    pub write:
        unsafe extern "C" fn(stream: *mut clap_ostream, buffer: *const c_void, size: u64) -> i64,
}
