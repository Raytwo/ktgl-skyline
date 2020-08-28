use std::ops::{ Index, IndexMut };

pub mod fixed_data;
pub mod fixed_persondata;

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
