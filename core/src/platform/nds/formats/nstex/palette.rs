use binrw::binrw;
use std::sync::Arc;

pub struct NsPalette {
    pub name: String,
    pub offset: usize,
    pub palette_data: Arc<Vec<u8>>,
}

#[binrw]
#[brw(little)]
pub struct PaletteParams {
    pub pltt_base: u16,
    pub unknown: u16,
}

impl PaletteParams {
    pub fn palette_data_offset(&self) -> usize {
        ((self.pltt_base & 0x1FFF) as usize) << 3
    }
}
