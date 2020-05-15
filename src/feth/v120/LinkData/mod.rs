use skyline::from_offset;
use skyline::libc::{ c_void, c_char };

#[from_offset(0x4A4700)]
pub fn get_uncompressed_size_data(file_id: u32) -> u64;

#[from_offset(0x4A0B40)]
pub fn get_uncompressed_size(linkdata_mgr: *const c_void, file_id: u32) -> u64;

#[from_offset(0x4A47D0)]
pub fn info_get_filepath(out_path: c_char, file_id: u32);