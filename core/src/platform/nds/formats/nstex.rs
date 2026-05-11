use crate::platform::nds::formats::nstex::palette::{NsPalette, PaletteParams};
use crate::platform::nds::formats::nstex::texture::{NsTexture, TextureFormat, TextureParams};
use crate::platform::nds::rom::NdsRomReadError;
use crate::rom::RomReadError;
use crate::rw::blob::read_blob;
use crate::rw::nds_dict::read_nds_dict;
use binrw::{binrw, BinRead};
use std::io::{Read, Seek, SeekFrom};
use std::sync::Arc;

pub mod palette;
pub mod texture;

pub struct Nstex {
    pub header: NstexHeader,
    pub textures: Vec<NsTexture>,
    pub palettes: Vec<NsPalette>,
}

impl Nstex {
    pub fn probe<R: Read + Seek>(reader: &mut R) -> Result<bool, RomReadError> {
        let pos = reader.stream_position()?;
        let result: Result<bool, std::io::Error> = (|| {
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            Ok(&buf == b"TEX0")
        })();
        reader.seek(SeekFrom::Start(pos))?;
        match result {
            Ok(v) => Ok(v),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError> {
        let base = reader.stream_position()?;
        let header = NstexHeader::read(reader)?;

        reader.seek(SeekFrom::Start(base + header.texture_dict_offset as u64))?;
        let texture_dict = read_nds_dict::<_, TextureParams>(reader)?;

        reader.seek(SeekFrom::Start(base + header.palette_dict_offset as u64))?;
        let palette_dict = read_nds_dict::<_, PaletteParams>(reader)?;

        let texture_data = read_blob(
            reader,
            base + header.texture_data_offset as u64,
            header.texture_data_size as usize * 8,
        )?;

        let compressed_texel_data = read_blob(
            reader,
            base + header.compressed_texture_offset_texel_data as u64,
            header.compressed_texture_data_size as usize * 12,
        )?;

        let compressed_texel_attr = read_blob(
            reader,
            base + header.compressed_texture_offset_texel_attr as u64,
            header.compressed_texture_data_size as usize * 4,
        )?;

        let palette_data = Arc::new(read_blob(
            reader,
            base + header.palette_data_offset as u64,
            header.palette_data_size as usize * 8,
        )?);

        reader.seek(SeekFrom::Start(base + header.chunk_size as u64))?;

        let textures = texture_dict
            .into_iter()
            .map(|(name, params)| {
                let format = TextureFormat::from_raw(params.format())
                    .ok_or_else(|| NdsRomReadError::InvalidTextureFormat(params.format()))?;
                let w = params.width();
                let h = params.height();
                let len = format.byte_len(w, h);
                let off = params.texture_data_offset();

                let (data1, data2) = if format.is_compressed() {
                    let d1 = compressed_texel_data[off..off + len].to_vec();
                    let d2 = compressed_texel_attr[off / 2..off / 2 + len / 2].to_vec();
                    (d1, d2)
                } else {
                    let d1 = texture_data[off..off + len].to_vec();
                    (d1, vec![])
                };

                Ok(NsTexture {
                    name,
                    params,
                    data1,
                    data2,
                })
            })
            .collect::<Result<Vec<_>, RomReadError>>()?;

        let palettes: Vec<NsPalette> = palette_dict
            .into_iter()
            .map(|(name, params)| NsPalette {
                name,
                offset: params.palette_data_offset(),
                palette_data: Arc::clone(&palette_data),
            })
            .collect();

        Ok(Self {
            header,
            textures,
            palettes,
        })
    }
}

#[binrw]
#[brw(little)]
pub struct NstexHeader {
    #[br(assert(magic == *b"TEX0"))]
    pub magic: [u8; 4],
    pub chunk_size: u32,
    pub _padding_1: [u8; 4],
    /// Texture data size / 8
    texture_data_size: u16,
    /// 0x03C
    pub texture_dict_offset: u16,
    pub _padding_2: [u8; 4],
    pub texture_data_offset: u32,
    pub _padding_3: [u8; 4],
    /// Compressed texture data size / 12
    compressed_texture_data_size: u16,
    /// 0x03C
    pub compressed_texture_dict_offset: u16,
    pub _padding_4: [u8; 4],
    pub compressed_texture_offset_texel_data: u32,
    pub compressed_texture_offset_texel_attr: u32,
    pub _padding_5: [u8; 4],
    /// Palette data size / 8
    palette_data_size: u32,
    pub palette_dict_offset: u32,
    pub palette_data_offset: u32,
}

impl NstexHeader {
    pub fn texture_data_size(&self) -> usize {
        (self.texture_data_size as usize) << 3
    }

    pub fn compressed_texture_data_size(&self) -> usize {
        (self.compressed_texture_data_size as usize) * 12
    }

    pub fn palette_data_size(&self) -> usize {
        (self.palette_data_size as usize) << 3
    }
}
