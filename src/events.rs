use crate::{fixedpoint::*, id::clap_id};

use std::ffi::c_void;

pub const CLAP_EVENT_NOTE_ON: clap_event_type = 0;
pub const CLAP_EVENT_NOTE_OFF: clap_event_type = 1;
pub const CLAP_EVENT_NOTE_END: clap_event_type = 2;
pub const CLAP_EVENT_NOTE_CHOKE: clap_event_type = 3;
pub const CLAP_EVENT_NOTE_EXPRESSION: clap_event_type = 4;
pub const CLAP_EVENT_NOTE_MASK: clap_event_type = 5;
pub const CLAP_EVENT_PARAM_VALUE: clap_event_type = 6;
pub const CLAP_EVENT_PARAM_MOD: clap_event_type = 7;
pub const CLAP_EVENT_TRANSPORT: clap_event_type = 8;
pub const CLAP_EVENT_MIDI: clap_event_type = 9;
pub const CLAP_EVENT_MIDI_SYSEX: clap_event_type = 10;

pub type clap_event_type = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event_note {
    pub port_index: i32,
    pub key: i32,
    pub channel: i32,
    pub velocity: f64,
}

pub const CLAP_NOTE_EXPRESSION_VOLUME: clap_note_expression = 0;
pub const CLAP_NOTE_EXPRESSION_PAN: clap_note_expression = 1;
pub const CLAP_NOTE_EXPRESSION_TUNING: clap_note_expression = 2;
pub const CLAP_NOTE_EXPRESSION_VIBRATO: clap_note_expression = 3;
pub const CLAP_NOTE_EXPRESSION_BRIGHTNESS: clap_note_expression = 4;
pub const CLAP_NOTE_EXPRESSION_BREATH: clap_note_expression = 5;
pub const CLAP_NOTE_EXPRESSION_PRESSURE: clap_note_expression = 6;
pub const CLAP_NOTE_EXPRESSION_TIMBRE: clap_note_expression = 7;

pub type clap_note_expression = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event_note_expression {
    pub expression_id: clap_note_expression,
    pub port_index: i32,
    pub key: i32,
    pub channel: i32,
    pub value: f64,
}

pub const CLAP_EVENT_PARAM_BEGIN_ADJUST: clap_event_param_flags = 1 << 0;
pub const CLAP_EVENT_PARAM_END_ADJUST: clap_event_param_flags = 1 << 1;
pub const CLAP_EVENT_PARAM_SHOULD_RECORD: clap_event_param_flags = 1 << 2;

pub type clap_event_param_flags = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event_param_value {
    pub cookie: *mut c_void,
    pub param_id: clap_id,
    pub port_index: i32,
    pub key: i32,
    pub channel: i32,
    pub flags: clap_event_param_flags,
    pub value: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event_param_mod {
    pub cookie: *mut c_void,
    pub param_id: clap_id,
    pub port_index: i32,
    pub key: i32,
    pub channel: i32,
    pub amount: f64,
}

pub const CLAP_TRANSPORT_HAS_TEMPO: clap_transport_flags = 1 << 0;
pub const CLAP_TRANSPORT_HAS_BEATS_TIMELINE: clap_transport_flags = 1 << 1;
pub const CLAP_TRANSPORT_HAS_SECONDS_TIMELINE: clap_transport_flags = 1 << 2;
pub const CLAP_TRANSPORT_HAS_TIME_SIGNATURE: clap_transport_flags = 1 << 3;
pub const CLAP_TRANSPORT_IS_PLAYING: clap_transport_flags = 1 << 4;
pub const CLAP_TRANSPORT_IS_RECORDING: clap_transport_flags = 1 << 5;
pub const CLAP_TRANSPORT_IS_LOOP_ACTIVE: clap_transport_flags = 1 << 6;
pub const CLAP_TRANSPORT_IS_WITHIN_PRE_ROLL: clap_transport_flags = 1 << 7;

pub type clap_transport_flags = u32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event_transport {
    pub flags: clap_transport_flags,
    pub song_pos_beats: clap_beattime,
    pub song_pos_seconds: clap_sectime,
    pub tempo: f64,
    pub tempo_inc: f64,
    pub bar_start: clap_beattime,
    pub bar_number: i32,
    pub loop_start_beats: clap_beattime,
    pub loop_end_beats: clap_beattime,
    pub loop_start_seconds: clap_sectime,
    pub loop_end_seconds: clap_sectime,
    pub tsig_num: i16,
    pub tsig_denom: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event_note_mask {
    pub port_index: i32,
    pub note_mask: u16,
    pub root_note: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event_midi {
    pub port_index: i32,
    pub data: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event_midi_sysex {
    pub port_index: i32,
    pub buffer: *const u8,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union clap_event_data {
    pub note: clap_event_note,
    pub note_expression: clap_event_note_expression,
    pub param_value: clap_event_param_value,
    pub param_mod: clap_event_param_mod,
    pub time_info: clap_event_transport,
    pub note_mask: clap_event_note_mask,
    pub midi: clap_event_midi,
    pub midi_sysex: clap_event_midi_sysex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event {
    pub type_: clap_event_type,
    pub time: u32,
    pub data: clap_event_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_event_list {
    pub ctx: *mut c_void,
    pub size: unsafe extern "C" fn(list: *const clap_event_list) -> u32,
    pub get: unsafe extern "C" fn(list: *const clap_event_list, index: u32) -> *const clap_event,
    pub push_back: unsafe extern "C" fn(list: *const clap_event_list, event: *const clap_event),
}
