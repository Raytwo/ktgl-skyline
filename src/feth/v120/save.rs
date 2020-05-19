use skyline::from_offset;

// TODO
#[repr(C)]
pub struct CharacterTable {
    vtable: *mut u64,
    pub entries: [CharacterTableEntry; 1201],
}

#[repr(C)]
pub struct CharacterTableEntry {
    unk1: *mut u64,
    pub block: *mut CharacterSectionBlock,
    pub index: u64,
}

#[repr(C)]
pub struct CharacterSectionBlock {
    pub post_timeskip_model_scale: f32,
}

#[from_offset(0x3CAF30)]
fn find_character_by_id(
    save_structure: *const skyline::libc::c_void,
    character_id: u32,
) -> *mut skyline::libc::c_void;
