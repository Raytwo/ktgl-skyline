use skyline::from_offset;
use skyline::libc::{ c_char };

#[from_offset(0x5EA6F0)]
pub fn show_popup_window(message: *const c_char, sfx_id: u32, unk3: u64, unk4: u64, unk5: u32) -> u64;

#[from_offset(0x3F7EA0)]
pub fn get_message_by_id(message_id: u32) -> *const c_char;
