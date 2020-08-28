/// "ts" in variable names stands for TimeSkip
#[repr(C)]
pub struct PersonBlock {
    pub chest_width_modifier: f32,
    pub before_ts_female_chest_size: f32,
    pub model_size_scale: f32,
    pub after_ts_female_chest_size: f32,
    unk1: u16,
    pub name: u16,
    unk2: u16,
    pub character_id: u16,
    pub asset_id: u16,
    pub class_id: u8,
    pub age: u8,
    pub birth_month: u8,
    pub birthday_flag: bool,
    pub birth_day: u8,
    unk3: u8,
    pub savedata_id: u8,
    unk4: u8,
    pub max_hp: u8,
    unk5: u8,
    pub allegiance: u8,
    unk6: u8,
    pub gender: PersonGender,
    pub body_type: u8,
    pub base_battalion: u8,
    pub hp_growth: u8,
    pub non_combat_anim_set: u8,
    pub base_hp: u8,
    pub primary_crest: u8,
    pub secondary_crest: u8,
    unk7: u8,
    pub before_ts_height: u8,
    pub after_ts_height: u8,
    unk8: u16,
    pub character_stats: PersonStats,
    padding: u16
}

#[repr(C)]
pub struct PersonStats {
    pub base_stats: Stats,
    pub base_growths: Stats,
    pub max_stats: Stats
}

#[repr(C)]
pub struct Stats {
    pub strengh: u8,
    pub magic: u8,
    pub dexterity: u8,
    pub speed: u8,
    pub luck: u8,
    pub defense: u8,
    pub resistance: u8,
    pub movement: u8,
    pub charm: u8
}

#[repr(u8)]
pub enum PersonGender {
    Male,
    Female
}