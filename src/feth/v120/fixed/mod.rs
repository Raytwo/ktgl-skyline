use std::ops::{ Index, IndexMut };

pub mod fixed_data;
pub mod fixed_persondata;

/// Make sure that the structure passed as <T> uses the #[repr(C)] layout
#[repr(C)]
pub struct FixedTable<'a, T, const N: usize> {
    vtable: *const (),
    pub entries: [FixedTableEntry<'a, T>; N],
    pub fixed_section: &'a FixedSection<T, N>,
    pub section_entries: &'a [T; N],
    end_of_section: *const u64
}

impl<'a, T, const N: usize> FixedTable<'a, T, N> {
    
}

#[repr(C)]
pub struct FixedSection<T, const N: usize> {
    pub header: FixedSectionHeader,
    pub entries: [T; N],
}

#[repr(C)]
pub struct FixedSectionHeader {
    pub magic: [u8;4],
    pub block_count: u32,
    pub block_size: u32,
    padding: [u8; 52],
}

// This can't enforce the strict array size of a FixedTable instance so this will have to go unused for now

// impl<'a, T, const N: usize> Index<usize> for FixedTable<'a, T, N> {
//     type Output = FixedTableEntry<'a, T>;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.entries.get(index).unwrap()
//     }
// }

// impl<'a, T, const N: usize> IndexMut<usize> for FixedTable<'a, T, N> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         self.entries.get_mut(index).unwrap()
//     }
// }

#[repr(C)]
pub struct FixedTableEntry<'a, T> {
    some_ptr: *const (),
    pub entry: &'a mut T,
    pub index: u64
}
