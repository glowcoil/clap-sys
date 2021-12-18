use clap_sys::{host::*, plugin::*, version::*};

use std::os::raw::c_char;
use std::ptr;

mod entry {
    use super::*;

    pub unsafe extern "C" fn init(_plugin_path: *const c_char) {}

    pub unsafe extern "C" fn deinit() {}

    pub unsafe extern "C" fn get_plugin_count() -> u32 {
        0
    }

    pub unsafe extern "C" fn get_plugin_descriptor(_index: u32) -> *const clap_plugin_descriptor {
        ptr::null()
    }

    pub unsafe extern "C" fn create_plugin(
        _host: *const clap_host,
        _plugin_id: *const c_char,
    ) -> *const clap_plugin {
        ptr::null()
    }

    pub unsafe extern "C" fn get_invalidation_source_count() -> u32 {
        0
    }

    pub unsafe extern "C" fn get_invalidation_source(
        _index: u32,
    ) -> *const clap_plugin_invalidation_source {
        ptr::null()
    }

    pub unsafe extern "C" fn refresh() {}
}

#[allow(non_upper_case_globals)]
#[no_mangle]
static clap_plugin_entry: clap_plugin_entry = clap_plugin_entry {
    clap_version: CLAP_VERSION,
    init: entry::init,
    deinit: entry::deinit,
    get_plugin_count: entry::get_plugin_count,
    get_plugin_descriptor: entry::get_plugin_descriptor,
    create_plugin: entry::create_plugin,
    get_invalidation_source_count: entry::get_invalidation_source_count,
    get_invalidation_source: entry::get_invalidation_source,
    refresh: entry::refresh,
};
