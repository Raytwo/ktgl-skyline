/// "ts" in variable names stands for TimeSkip
#[repr(C)]
pub struct PersonBlock {
    pub before_ts_model_size_scale: f32,
    pub before_ts_female_chest_size: f32,
    pub after_ts_model_size_scale: f32,
    pub after_ts_female_chest_size: f32,
    unk1: u16,
    pub name: u16,
    unk2: u16,
    pub voice_id: u16,
    pub asset_id: u16,
    pub class_id: Class,
    pub age: u8,
    pub birth_month: u8,
    pub birthday_flag: u8,
    pub birth_day: u8,
    unk3: u8,
    pub savedata_id: u8,
    unk4: u8,
    pub max_hp: u8,
    unk5: u8,
    pub allegiance: AllegianceId,
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

#[repr(u8)]
pub enum Class
{
    Noble,
    Commoner,
    Myrmidon,
    Soldier,
    Fighter,
    Monk,
    Lord,
    Mercenary,
    Thief,
    ArmoredKnight,
    Cavalier,
    Brigand,
    Archer,
    Brawler,
    Mage,
    DarkMage,
    Priest,
    Barbarossa,
    Hero,
    Swordmaster,
    Assassin,
    FortressKnight,
    Paladin,
    PegasusKnight,
    WyvernRider,
    Warrior,
    Sniper,
    Grappler,
    Warlock,
    DarkBishop,
    Bishop,
    FalconKnight,
    WyvernLord,
    MortalSavant,
    GreatKnight,
    BowKnight,
    DarkKnight,
    HolyKnight,
    WarMaster,
    Gremory,
    Emperor,
    Agastya,
    EnlightenedOne,
    Dancer,
    GreatLord,
    KingOfLiberation,
    Saint,
    FlameEmperor,
    FlameEmperor2,
    Thief2,
    Ruffian,
    Paladin2,
    FortressKnight2,
    Lord2,
    PegasusKnight2,
    Archbishop,
    ArmoredLord,
    HighLord,
    WyvernMaster,
    DeathKnight,
    BlackBeast,
    WanderingBeast,
    WildDemonicBeast,
    DemonicBeast,
    ExpDemonicBeast,
    AlteredDemonicBeast,
    GiantDemonicBeast,
    FlyingDemonicBeast,
    Golem,
    AlteredGolem,
    Titanus,
    WhiteBeast,
    TheImmaculateOne,
    TheImmaculateOne2,
    TheImmaculateOne3,
    LordOfTheDesert,
    LordOftheLake,
    GiantBird,
    GiantCrawler,
    GiantWolf,
    HegemonHusk,
    KingOfBeasts,
    KingOfFangs,
    KingOfWings,
	Trickster,
	WarMonk,
	DarkFlier,
	MageKnight,
    DeathKnight2,
    ID92,
    ID93,
    ID94,
    ID95,
    ID96,
    ID97,
    ID98,
    ID99,
    None = 255,
}

#[repr(u8)]
pub enum AllegianceId
{
	ChurchOfSeiros = 0,
	AdrestianEmpire = 1,
	HolyKingdomOfFaerghus = 2,
	LeicesterAlliance = 3,
	ThoseWhoSlitherIntheDark = 4,
	Almyra = 5,
	WesternChurch = 6,
	ResistanceArmy = 7,
	FlameEmperorArmy = 8,
	Thieves = 9,
	RebelArmy = 10,
	Rogues = 11,
	Citizens = 12,
	ChildrenOfTheGoddess = 13,
	GiantBeasts = 14,
	Independents = 15,
	Mercenaries = 16,
	LiberationArmy = 17,
	Bandits = 18,
	Pirates = 19,
	FaerghusDukedom = 20,
	OldFaerghusTerritories = 21,
}