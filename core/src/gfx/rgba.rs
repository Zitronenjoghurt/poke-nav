use bytemuck::{Pod, Zeroable};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Pod, Zeroable)]
#[repr(transparent)]
pub struct Rgba([u8; 4]);

impl Rgba {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b, 255])
    }

    #[inline]
    pub fn r(&self) -> u8 {
        self.0[0]
    }

    #[inline]
    pub fn g(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn b(&self) -> u8 {
        self.0[2]
    }

    #[inline]
    pub fn a(&self) -> u8 {
        self.0[3]
    }
}

impl From<[u8; 4]> for Rgba {
    fn from(arr: [u8; 4]) -> Self {
        Self(arr)
    }
}

impl From<[u8; 3]> for Rgba {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self::from_rgb(r, g, b)
    }
}

impl From<Rgba> for [u8; 4] {
    fn from(c: Rgba) -> Self {
        c.0
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct RgbaBuffer {
    width: u32,
    height: u32,
    pixels: Vec<Rgba>,
}

impl RgbaBuffer {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Rgba::default(); width as usize * height as usize],
        }
    }

    #[inline]
    pub fn from_raw(width: u32, height: u32, bytes: Vec<u8>) -> Option<Self> {
        if bytes.len() != (width * height * 4) as usize {
            return None;
        };
        Some(Self {
            width,
            height,
            pixels: bytemuck::allocation::cast_vec(bytes),
        })
    }

    #[inline]
    pub fn into_raw(self) -> Vec<u8> {
        bytemuck::allocation::cast_vec(self.pixels)
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }

    #[inline]
    fn coords_to_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    #[inline]
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<Rgba> {
        self.pixels.get(self.coords_to_index(x, y)).copied()
    }

    #[inline]
    pub fn get_pixel_mut(&mut self, x: u32, y: u32) -> Option<&mut Rgba> {
        let index = self.coords_to_index(x, y);
        self.pixels.get_mut(index)
    }

    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: Rgba) {
        if let Some(p) = self.get_pixel_mut(x, y) {
            *p = pixel;
        }
    }

    #[inline]
    pub fn get_pixel_pos(&self, pos: usize) -> Option<Rgba> {
        self.pixels.get(pos).copied()
    }

    #[inline]
    pub fn get_pixel_pos_mut(&mut self, pos: usize) -> Option<&mut Rgba> {
        self.pixels.get_mut(pos)
    }

    #[inline]
    pub fn set_pixel_pos(&mut self, pos: usize, pixel: Rgba) {
        if let Some(p) = self.get_pixel_pos_mut(pos) {
            *p = pixel;
        }
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        bytemuck::cast_slice(&self.pixels)
    }

    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        bytemuck::cast_slice_mut(&mut self.pixels)
    }

    #[inline]
    pub fn row(&self, y: u32) -> &[Rgba] {
        let start = (y * self.width) as usize;
        &self.pixels[start..start + self.width as usize]
    }

    #[inline]
    pub fn row_mut(&mut self, y: u32) -> &mut [Rgba] {
        let start = (y * self.width) as usize;
        &mut self.pixels[start..start + self.width as usize]
    }

    #[cfg(feature = "png")]
    pub fn render_png(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buf = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut buf, self.width, self.height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header()?;
            writer.write_image_data(self.as_bytes())?;
        }
        Ok(buf)
    }
}

pub struct RgbaBufferWriter<'a> {
    buffer: &'a mut RgbaBuffer,
    pos: usize,
}

impl<'a> RgbaBufferWriter<'a> {
    #[inline]
    pub fn new(buffer: &'a mut RgbaBuffer) -> Self {
        Self { buffer, pos: 0 }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.buffer.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.buffer.height
    }

    #[inline]
    pub fn write(&mut self, pixel: Rgba) -> bool {
        if self.pos >= self.buffer.pixels.len() {
            return false;
        }
        self.buffer.pixels[self.pos] = pixel;
        self.pos += 1;
        true
    }

    #[inline]
    pub fn write_row(&mut self, row: &[Rgba]) {
        let end = (self.pos + row.len()).min(self.buffer.len());
        let count = end - self.pos;
        self.buffer.pixels[self.pos..self.pos + count].copy_from_slice(row);
        self.pos += count;
    }

    #[inline]
    pub fn remaining(&self) -> usize {
        self.buffer.pixels.len() - self.pos
    }

    #[inline]
    pub fn is_done(&self) -> bool {
        self.pos >= self.buffer.pixels.len()
    }
}
