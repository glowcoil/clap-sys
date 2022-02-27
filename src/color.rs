#[repr(C)]
#[derive(Copy, Clone)]
pub struct clap_color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
