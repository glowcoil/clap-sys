use crate::plugin::*;

use std::os::raw::c_char;

pub const CLAP_EXT_RENDER: *const c_char = b"clap.render\0".as_ptr() as *const c_char;

pub const CLAP_RENDER_REALTIME: clap_plugin_render_mode = 0;
pub const CLAP_RENDER_OFFLINE: clap_plugin_render_mode = 1;

pub type clap_plugin_render_mode = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_plugin_render {
    pub set: unsafe extern "C" fn(plugin: *const clap_plugin, mode: clap_plugin_render_mode),
}
