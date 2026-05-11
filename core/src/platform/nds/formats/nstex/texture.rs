use binrw::binrw;

pub struct NsTexture {
    pub name: String,
    pub params: TextureParams,
    pub data1: Vec<u8>,
    pub data2: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TextureFormat {
    A3I5 = 1,
    Palette4 = 2,
    Palette16 = 3,
    Palette256 = 4,
    Compressed = 5,
    A5I3 = 6,
    Direct = 7,
}

impl TextureFormat {
    pub fn from_raw(val: u8) -> Option<Self> {
        match val {
            1 => Some(Self::A3I5),
            2 => Some(Self::Palette4),
            3 => Some(Self::Palette16),
            4 => Some(Self::Palette256),
            5 => Some(Self::Compressed),
            6 => Some(Self::A5I3),
            7 => Some(Self::Direct),
            _ => None,
        }
    }

    pub fn bpp(self) -> u32 {
        match self {
            Self::A3I5 | Self::Palette256 | Self::A5I3 => 8,
            Self::Palette4 | Self::Compressed => 2,
            Self::Palette16 => 4,
            Self::Direct => 16,
        }
    }

    pub fn byte_len(self, w: u32, h: u32) -> usize {
        (w as usize * h as usize * self.bpp() as usize) / 8
    }

    pub fn requires_palette(self) -> bool {
        !matches!(self, Self::Direct)
    }

    pub fn is_compressed(self) -> bool {
        matches!(self, Self::Compressed)
    }
}

#[binrw]
#[brw(little)]
pub struct TextureParams {
    pub image_params: u32,
    pub width_height: u32,
}

impl TextureParams {
    pub fn texture_data_offset(&self) -> usize {
        (((self.image_params & 0x0000FFFF) as u16) as usize) << 3
    }

    pub fn s_size(&self) -> u8 {
        ((self.image_params >> 20) & 0x7) as u8
    }

    pub fn t_size(&self) -> u8 {
        ((self.image_params >> 23) & 0x7) as u8
    }

    pub fn format(&self) -> u8 {
        ((self.image_params >> 26) & 0x7) as u8
    }

    pub fn is_color_0_transparent(&self) -> bool {
        ((self.image_params >> 29) & 0x1) != 0
    }

    pub fn width(&self) -> u32 {
        self.width_height & 0x7FF
    }

    pub fn height(&self) -> u32 {
        (self.width_height >> 11) & 0x7FF
    }
}
