use skyline::from_offset;

// TODO
#[repr(C)]
pub struct Character {
    test: u32,
}

#[from_offset(0x3CAF30)]
fn find_character_by_id(
    save_structure: *const skyline::libc::c_void,
    character_id: u32,
) -> *mut skyline::libc::c_void;
