use skyline::libc;

#[skyline::from_offset(0x540b30)]
pub fn kt_malloc(size: u32) -> *const skyline::libc::c_void;

#[skyline::from_offset(0x5BAB80)]
pub fn kt_aligned_malloc(size: libc::size_t, align: u64) -> *const libc::c_void;

#[skyline::from_offset(0x540b50)]
pub fn kt_free(pointer: *mut skyline::libc::c_void) -> *const skyline::libc::c_void;

#[skyline::from_offset(0x540b70)]
pub fn kt_realloc(pointer: *mut skyline::libc::c_void, size: libc::size_t) -> *const skyline::libc::c_void;

#[skyline::from_offset(0x540b90)]
pub fn kt_calloc(t1: u32, t2: u32) -> *const skyline::libc::c_void;