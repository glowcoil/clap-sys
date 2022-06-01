use crate::{id::*, stream::*};

use std::os::raw::c_char;

pub const CLAP_CLAP_CONVERTER_FACTORY_ID: *const c_char =
    b"clap.clap-converter-factory/draft0\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_clap_converter {
    pub src_plugin_id: *const c_char,
    pub dst_plugin_id: *const c_char,

    pub convert_state: unsafe extern "C" fn(
        converter: *const clap_clap_converter,
        src: *const clap_istream,
        dst: *const clap_ostream,
    ) -> bool,
    pub convert_normalized_value: unsafe extern "C" fn(
        converter: *const clap_clap_converter,
        src_param_id: clap_id,
        src_normalized_value: f64,
        dst_param_id: *mut clap_id,
        dst_normalized_value: *mut f64,
    ) -> bool,
    pub convert_plain_value: unsafe extern "C" fn(
        converter: *const clap_clap_converter,
        src_param_id: clap_id,
        src_plain_value: f64,
        dst_param_id: *mut clap_id,
        dst_plain_value: *mut f64,
    ) -> bool,
}

unsafe impl Send for clap_clap_converter {}
unsafe impl Sync for clap_clap_converter {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_clap_converter_factory {
    pub count: unsafe extern "C" fn(factory: *const clap_clap_converter_factory) -> u32,
    pub get: unsafe extern "C" fn(
        factory: *const clap_clap_converter_factory,
        index: u32,
    ) -> *const clap_clap_converter,
}
