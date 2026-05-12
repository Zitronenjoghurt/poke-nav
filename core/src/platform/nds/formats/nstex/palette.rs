use crate::gfx::rgb555::Rgb555;
use binrw::binrw;
use std::sync::Arc;

pub struct NsPalette {
    pub name: String,
    pub offset: usize,
    pub palette_data: Arc<Vec<u8>>,
}

impl NsPalette {
    pub fn colors(&'_ self) -> NsPaletteColors<'_> {
        let bytes = &self.palette_data[self.offset..];
        NsPaletteColors {
            colors: bytemuck::cast_slice(bytes),
        }
    }
}

#[binrw]
#[brw(little)]
pub struct PaletteParams {
    pub pltt_base: u16,
    pub unknown: u16,
}

impl PaletteParams {
    pub fn data_offset(&self) -> usize {
        ((self.pltt_base & 0x1FFF) as usize) << 3
    }
}

pub struct NsPaletteColors<'a> {
    colors: &'a [Rgb555],
}

impl<'a> NsPaletteColors<'a> {
    #[inline]
    pub fn get(&self, index: usize) -> Option<Rgb555> {
        self.colors.get(index).copied()
    }
}
