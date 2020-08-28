use std::ops::{ Index, IndexMut };

/// Make sure that the structure passed as <T> uses the #[repr(C)] layout
#[repr(C)]
pub struct FixedTable<'a, T, const N: usize> {
    vtable: *const (),
    pub entries: [FixedTableEntry<'a, T>; N],
    bingz_entry: *const (),
    table: *const T,
    eos: *const ()
}

impl<'a, T, const N: usize> FixedTable<'a, T, N> {
    
}

impl<'a, T, const N: usize> Index<usize> for FixedTable<'a, T, N> {
    type Output = FixedTableEntry<'a, T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.entries.get(index).unwrap()
    }
}

impl<'a, T, const N: usize> IndexMut<usize> for FixedTable<'a, T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.entries.get_mut(index).unwrap()
    }
}

#[repr(C)]
pub struct FixedTableEntry<'a, T> {
    some_ptr: *const (),
    pub entry: &'a mut T,
    pub index: u64
}

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
