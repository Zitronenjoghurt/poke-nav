use crate::gfx::rgba::Rgba;
use bytemuck::{Pod, Zeroable};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Pod, Zeroable)]
#[repr(transparent)]
pub struct Rgb555(u16);

impl Rgb555 {
    #[inline]
    pub fn from_le(rgb555: u16) -> Self {
        Self(rgb555)
    }

    #[inline]
    pub fn r(&self) -> u8 {
        (self.0 & 0x1F) as u8
    }

    #[inline]
    pub fn g(&self) -> u8 {
        ((self.0 >> 5) & 0x1F) as u8
    }

    #[inline]
    pub fn b(&self) -> u8 {
        ((self.0 >> 10) & 0x1F) as u8
    }

    #[inline]
    pub fn to_rgba_transparent(self) -> Rgba {
        Rgba::new(
            extend_5bit_to_8bit(self.r()),
            extend_5bit_to_8bit(self.g()),
            extend_5bit_to_8bit(self.b()),
            0,
        )
    }

    #[inline]
    pub fn to_rgba_opaque(self) -> Rgba {
        Rgba::new(
            extend_5bit_to_8bit(self.r()),
            extend_5bit_to_8bit(self.g()),
            extend_5bit_to_8bit(self.b()),
            255,
        )
    }

    #[inline]
    pub fn to_rgba_with_a5(self, a5: u8) -> Rgba {
        Rgba::new(
            extend_5bit_to_8bit(self.r()),
            extend_5bit_to_8bit(self.g()),
            extend_5bit_to_8bit(self.b()),
            extend_5bit_to_8bit(a5),
        )
    }

    #[inline]
    pub fn to_rgba_with_alpha_bit(self) -> Rgba {
        if self.0 & 0x8000 != 0 {
            self.to_rgba_opaque()
        } else {
            self.to_rgba_transparent()
        }
    }
}

#[inline]
fn extend_5bit_to_8bit(x: u8) -> u8 {
    (x << 3) | (x >> 2)
}
