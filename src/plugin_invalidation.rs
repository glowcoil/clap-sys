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

pub const CLAP_PLUGIN_INVALIDATION_FACTORY_ID: *const c_char =
    b"clap.plugin-invalidation-factory/draft0\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct clap_plugin_invalidation_factory {
    pub count: unsafe extern "C" fn(factory: *const clap_plugin_invalidation_factory) -> u32,
    pub get: unsafe extern "C" fn(
        factory: *const clap_plugin_invalidation_factory,
        index: u32,
    ) -> *const clap_plugin_invalidation_source,
    pub refresh: unsafe extern "C" fn(factory: *const clap_plugin_invalidation_factory) -> bool,
}
