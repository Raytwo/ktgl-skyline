use skyline::from_offset;
use skyline::libc::{ c_char };

#[from_offset(0x5EA6F0)]
pub fn show_popup_window(message: *const c_char, sfx_id: u32, unk3: u64, unk4: u64, unk5: u32) -> u64;

#[from_offset(0x3F7EA0)]
pub fn get_message_by_id(message_id: u32) -> *const c_char;

#[from_offset(0x5C5E40)]
pub fn display_text(pos_x: f32, pos_y: f32, scale_x: f32, scale_y: f32, text: *const c_char, box_clipping: i32, unk1: u32, centered: bool, unk2: i32, unk3: i32, shading_effect: i8, alpha: i8);
