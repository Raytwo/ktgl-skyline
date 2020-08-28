#[repr(C)]
pub struct ItemBlock {
    unk1: [u8; 5],
    pub weapon_type: u8,
    pub rank: u8,
    pub max_range: u8,
    pub weapon_model_id: u8,
    pub crests: u8,
    pub min_range: u8,
    pub uses: u8,
    pub extra_effects: u8,
    pub item_type: u8,
    pub effectiveness: u8,
    unk2: u8,
    pub increase: u8,
    pub stat: u8,
    pub crit: u8,
    pub weight: u8,
    pub flags: [u8; 3],
    padding: u8,
}