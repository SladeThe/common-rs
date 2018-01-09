pub const TB_PER_PB: usize = 1024;

pub const GB_PER_TB: usize = 1024;
pub const GB_PER_PB: usize = GB_PER_TB * TB_PER_PB;

pub const MB_PER_GB: usize = 1024;
pub const MB_PER_TB: usize = MB_PER_GB * GB_PER_TB;
pub const MB_PER_PB: usize = MB_PER_TB * TB_PER_PB;

pub const KB_PER_MB: usize = 1024;
pub const KB_PER_GB: usize = KB_PER_MB * MB_PER_GB;
pub const KB_PER_TB: usize = KB_PER_GB * GB_PER_TB;
pub const KB_PER_PB: usize = KB_PER_TB * TB_PER_PB;

pub const BYTES_PER_KB: usize = 1024;
pub const BYTES_PER_MB: usize = BYTES_PER_KB * KB_PER_MB;
pub const BYTES_PER_GB: usize = BYTES_PER_MB * MB_PER_GB;
pub const BYTES_PER_TB: usize = BYTES_PER_GB * GB_PER_TB;
pub const BYTES_PER_PB: usize = BYTES_PER_TB * TB_PER_PB;
