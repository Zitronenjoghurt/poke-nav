use crate::gfx::rgb555::Rgb555;
use crate::gfx::rgba::{RgbaBuffer, RgbaBufferWriter};
use crate::platform::nds::formats::nstex::palette::NsPalette;
use crate::platform::nds::formats::nstex::texture::{NsTexture, TextureFormat};

#[derive(Debug, thiserror::Error)]
pub enum NstexDecodeError {
    #[error("Missing palette")]
    MissingPalette,
    #[error("Invalid texture format: {0}")]
    InvalidTextureFormat(u8),
    #[error("Invalid palette index")]
    InvalidPaletteIndex,
    #[error("Invalid texture index")]
    InvalidTextureIndex,
}

pub fn decode_texture(
    tex: &NsTexture,
    pal: Option<&NsPalette>,
) -> Result<RgbaBuffer, NstexDecodeError> {
    let format = tex.format()?;
    if format.requires_palette() && pal.is_none() {
        return Err(NstexDecodeError::MissingPalette);
    }

    let (width, height) = tex.dimensions();
    let mut buf = RgbaBuffer::new(width, height);
    let mut w = RgbaBufferWriter::new(&mut buf);

    match format {
        TextureFormat::A3I5 => decode_format_1(&mut w, tex, pal.unwrap()),
        TextureFormat::Palette4 => decode_format_2(&mut w, tex, pal.unwrap()),
        TextureFormat::Palette16 => decode_format_3(&mut w, tex, pal.unwrap()),
        TextureFormat::Palette256 => decode_format_4(&mut w, tex, pal.unwrap()),
        TextureFormat::Compressed => decode_format_5(&mut w, tex, pal.unwrap()),
        TextureFormat::A5I3 => decode_format_6(&mut w, tex, pal.unwrap()),
        TextureFormat::Direct => decode_format_7(&mut w, tex),
        _ => unimplemented!(),
    }

    Ok(buf)
}

fn decode_format_1(w: &mut RgbaBufferWriter, tex: &NsTexture, pal: &NsPalette) {
    let colors = pal.colors();
    let num_texels = (w.width() * w.height()) as usize;

    for texel in &tex.data1[..num_texels] {
        let index = (texel & 0x1F) as usize;
        let color = colors.get(index).unwrap_or_default();

        let a = (texel >> 5) & 0x7;
        let alpha = (a << 2) | (a >> 1);

        let rgba = color.to_rgba_with_a5(alpha);
        w.write(rgba);
    }
}

fn decode_format_2(w: &mut RgbaBufferWriter, tex: &NsTexture, pal: &NsPalette) {
    let color0_transparent = tex.params.is_color_0_transparent();
    let colors = pal.colors();
    let num_bytes = TextureFormat::Palette16.byte_len(w.width(), w.height());

    for byte in &tex.data1[..num_bytes] {
        for nibble in [
            byte & 0x03,
            (byte >> 2) & 0x03,
            (byte >> 4) & 0x03,
            (byte >> 6) & 0x03,
        ] {
            let index = nibble as usize;
            let color = colors.get(index).unwrap_or_default();
            let rgba = if index == 0 && color0_transparent {
                color.to_rgba_transparent()
            } else {
                color.to_rgba_opaque()
            };
            w.write(rgba);
        }
    }
}

fn decode_format_3(w: &mut RgbaBufferWriter, tex: &NsTexture, pal: &NsPalette) {
    let color0_transparent = tex.params.is_color_0_transparent();
    let colors = pal.colors();
    let num_bytes = TextureFormat::Palette16.byte_len(w.width(), w.height());

    for byte in &tex.data1[..num_bytes] {
        for nibble in [byte & 0x0F, byte >> 4] {
            let index = nibble as usize;
            let color = colors.get(index).unwrap_or_default();
            let rgba = if index == 0 && color0_transparent {
                color.to_rgba_transparent()
            } else {
                color.to_rgba_opaque()
            };
            w.write(rgba);
        }
    }
}

fn decode_format_4(w: &mut RgbaBufferWriter, tex: &NsTexture, pal: &NsPalette) {
    let color0_transparent = tex.params.is_color_0_transparent();
    let colors = pal.colors();
    let num_bytes = TextureFormat::Palette256.byte_len(w.width(), w.height());

    for byte in &tex.data1[..num_bytes] {
        let index = *byte as usize;
        let color = colors.get(index).unwrap_or_default();
        let rgba = if index == 0 && color0_transparent {
            color.to_rgba_transparent()
        } else {
            color.to_rgba_opaque()
        };
        w.write(rgba);
    }
}

fn decode_format_5(w: &mut RgbaBufferWriter, tex: &NsTexture, pal: &NsPalette) {
    let width = w.width() as usize;
    let height = w.height() as usize;
    let num_blocks_x = width / 4;

    let blocks: &[u32] = bytemuck::cast_slice(&tex.data1);
    let extras: &[u16] = bytemuck::cast_slice(&tex.data2);

    let colors = pal.colors();

    for y in 0..height {
        for x in 0..width {
            let block_x = x / 4;
            let block_y = y / 4;
            let block_idx = block_y * num_blocks_x + block_x;

            let block = blocks[block_idx];
            let extra = extras[block_idx];

            let texel_off = 2 * (4 * (y % 4) + (x % 4));
            let texel = (block >> texel_off) & 0x03;

            let mode = (extra >> 14) & 0x03;
            let pal_addr = ((extra & 0x3FFF) as usize) << 1;

            let get_color = |n| -> [u8; 4] {
                colors
                    .get(pal_addr + n)
                    .unwrap_or_default()
                    .to_rgba_opaque()
                    .into()
            };

            let get_transparent = || -> [u8; 4] {
                colors
                    .get(0)
                    .unwrap_or_default()
                    .to_rgba_transparent()
                    .into()
            };

            let rgba: [u8; 4] = match (mode, texel) {
                (0, 0) => get_color(0),
                (0, 1) => get_color(1),
                (0, 2) => get_color(2),
                (0, 3) => get_transparent(),

                (1, 0) => get_color(0),
                (1, 1) => get_color(1),
                (1, 2) => avg(get_color(0), get_color(1)),
                (1, 3) => get_transparent(),

                (2, 0) => get_color(0),
                (2, 1) => get_color(1),
                (2, 2) => get_color(2),
                (2, 3) => get_color(3),

                (3, 0) => get_color(0),
                (3, 1) => get_color(1),
                (3, 2) => avg358(get_color(1), get_color(0)),
                (3, 3) => avg358(get_color(0), get_color(1)),

                _ => unreachable!(),
            };

            w.write(rgba.into());
        }
    }
}

fn decode_format_6(w: &mut RgbaBufferWriter, tex: &NsTexture, pal: &NsPalette) {
    let colors = pal.colors();
    let num_texels = (w.width() * w.height()) as usize;

    for texel in &tex.data1[..num_texels] {
        let index = (texel & 0x7) as usize;
        let alpha = (texel >> 3) & 0x01F;

        let color = colors.get(index).unwrap_or_default();
        let rgba = color.to_rgba_with_a5(alpha);
        w.write(rgba);
    }
}

fn decode_format_7(w: &mut RgbaBufferWriter, tex: &NsTexture) {
    let num_texels = (w.width() * w.height()) as usize;
    let texels: &[Rgb555] = bytemuck::cast_slice(&tex.data1[..num_texels]);
    for texel in texels {
        w.write(texel.to_rgba_with_alpha_bit());
    }
}

fn avg(c1: [u8; 4], c2: [u8; 4]) -> [u8; 4] {
    [
        ((c1[0] as u16 + c2[0] as u16) / 2) as u8,
        ((c1[1] as u16 + c2[1] as u16) / 2) as u8,
        ((c1[2] as u16 + c2[2] as u16) / 2) as u8,
        ((c1[3] as u16 + c2[3] as u16) / 2) as u8,
    ]
}

fn avg358(c1: [u8; 4], c2: [u8; 4]) -> [u8; 4] {
    [
        ((3 * c1[0] as u16 + 5 * c2[0] as u16) / 8) as u8,
        ((3 * c1[1] as u16 + 5 * c2[1] as u16) / 8) as u8,
        ((3 * c1[2] as u16 + 5 * c2[2] as u16) / 8) as u8,
        ((3 * c1[3] as u16 + 5 * c2[3] as u16) / 8) as u8,
    ]
}
