use crate::{id::*, stream::*};

use std::os::raw::c_char;

pub const CLAP_VST3_CONVERTER_FACTORY_ID: *const c_char =
    b"clap.vst3-converter-factory/draft0\0".as_ptr() as *const c_char;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_vst3_converter {
    pub vst3_plugin_tuid: [i8; 16],
    pub clap_plugin_id: *const c_char,

    pub convert_state: unsafe extern "C" fn(
        converter: *const clap_vst3_converter,
        vst3_processor: *const clap_istream,
        vst3_editor: *const clap_istream,
        clap: *const clap_ostream,
    ) -> bool,
    pub convert_normalized_value: unsafe extern "C" fn(
        converter: *const clap_vst3_converter,
        vst3_param_id: u32,
        vst3_normalized_value: f64,
        clap_param_id: *mut clap_id,
        clap_normalized_value: *mut f64,
    ) -> bool,
    pub convert_plain_value: unsafe extern "C" fn(
        converter: *const clap_vst3_converter,
        vst3_param_id: u32,
        vst3_plain_value: f64,
        clap_param_id: *mut clap_id,
        clap_plain_value: *mut f64,
    ) -> bool,
}

unsafe impl Send for clap_vst3_converter {}
unsafe impl Sync for clap_vst3_converter {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_vst3_converter_factory {
    pub count: unsafe extern "C" fn(factory: *const clap_vst3_converter_factory) -> u32,
    pub get: unsafe extern "C" fn(
        factory: *const clap_vst3_converter_factory,
        index: u32,
    ) -> *const clap_vst3_converter,
}
