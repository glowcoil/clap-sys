use crate::audio_buffer::*;
use crate::events::*;

pub const CLAP_PROCESS_ERROR: clap_process_status = 0;
pub const CLAP_PROCESS_CONTINUE: clap_process_status = 1;
pub const CLAP_PROCESS_CONTINUE_IF_NOT_QUIET: clap_process_status = 2;
pub const CLAP_PROCESS_SLEEP: clap_process_status = 3;

pub type clap_process_status = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_process {
    pub steady_time: i64,
    pub frames_count: u32,
    pub transport: *const clap_event_transport,
    pub audio_inputs: *const clap_audio_buffer,
    pub audio_outputs: *mut clap_audio_buffer,
    pub audio_inputs_count: u32,
    pub audio_outputs_count: u32,
    pub in_events: *const clap_input_events,
    pub out_events: *const clap_output_events,
}

unsafe impl Send for clap_process {}
unsafe impl Sync for clap_process {}
