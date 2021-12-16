#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_audio_buffer {
    pub data32: *const *const f32,
    pub data64: *const *const f64,
    pub channel_count: u32,
    pub latency: u32,
    pub constant_mask: u64,
}
