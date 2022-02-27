use crate::{host::*, plugin::*};

use std::os::raw::c_char;

pub const CLAP_PLUGIN_FACTORY_ID: *const c_char =
    b"clap.plugin-factory\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_factory {
    pub get_plugin_count: unsafe extern "C" fn(factory: *const clap_plugin_factory) -> u32,
    pub get_plugin_descriptor: unsafe extern "C" fn(
        factory: *const clap_plugin_factory,
        index: u32,
    ) -> *const clap_plugin_descriptor,
    pub create_plugin: unsafe extern "C" fn(
        factory: *const clap_plugin_factory,
        host: *const clap_host,
        plugin_id: *const c_char,
    ) -> *const clap_plugin,
}
