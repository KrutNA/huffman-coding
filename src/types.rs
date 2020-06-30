pub mod node;
pub mod table;
pub mod data;
pub mod ui;

pub const SHIFT:        u8    = 1;
pub const BYTE_SIZE:    usize = 8;
pub const BYTE_SIZE_U8: u8    = BYTE_SIZE as u8;
pub const BUFFER_SIZE:  usize = 1 << 20;
