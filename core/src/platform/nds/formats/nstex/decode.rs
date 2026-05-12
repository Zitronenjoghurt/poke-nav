use crate::gfx::rgba::{RgbaBuffer, RgbaBufferWriter};
use crate::platform::nds::formats::nstex::palette::NsPalette;
use crate::platform::nds::formats::nstex::texture::{NsTexture, TextureFormat};

#[derive(Debug, thiserror::Error)]
pub enum NstexDecodeError {
    #[error("Missing palette")]
    MissingPalette,
    #[error("Invalid texture format: {0}")]
    InvalidTextureFormat(u8),
    #[error("Index out of bounds")]
    InvalidTextureOrPaletteIndex,
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
        TextureFormat::Palette16 => decode_format_3(&mut w, tex, pal.unwrap()),
        _ => unimplemented!(),
    }

    Ok(buf)
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
