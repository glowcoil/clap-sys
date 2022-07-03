use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_invalidation_source {
    pub directory: *const c_char,
    pub filename_glob: *const c_char,
    pub recursive_scan: bool,
}

unsafe impl Send for clap_plugin_invalidation_source {}
unsafe impl Sync for clap_plugin_invalidation_source {}

pub const CLAP_PLUGIN_INVALIDATION_FACTORY_ID: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"clap.plugin-invalidation-factory/draft0\0") };

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_invalidation_factory {
    pub count:
        Option<unsafe extern "C" fn(factory: *const clap_plugin_invalidation_factory) -> u32>,
    pub get: Option<
        unsafe extern "C" fn(
            factory: *const clap_plugin_invalidation_factory,
            index: u32,
        ) -> *const clap_plugin_invalidation_source,
    >,
    pub refresh:
        Option<unsafe extern "C" fn(factory: *const clap_plugin_invalidation_factory) -> bool>,
}
