use crate::platform::nds::formats::nstex::Nstex;
use crate::rom::RomReadError;
use binrw::{binrw, BinRead};
use std::io::{Read, Seek, SeekFrom};

pub struct Nsbtx {
    pub header: NsbtxHeader,
    pub texture: Nstex,
}

impl Nsbtx {
    pub fn probe<R: Read + Seek>(reader: &mut R) -> Result<bool, RomReadError> {
        let pos = reader.stream_position()?;
        let result: Result<bool, std::io::Error> = (|| {
            let mut buf = [0u8; 16];
            reader.read_exact(&mut buf)?;
            Ok(&buf[0..4] == b"BTX0"
                && u16::from_le_bytes(buf[12..14].try_into().unwrap()) == 0x10
                && u16::from_le_bytes(buf[14..16].try_into().unwrap()) == 1)
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

        let header = NsbtxHeader::read(reader)?;

        reader.seek(SeekFrom::Start(base + header.offset as u64))?;
        let nstex = Nstex::read(reader)?;

        Ok(Self {
            header,
            texture: nstex,
        })
    }
}

#[binrw]
#[brw(little)]
pub struct NsbtxHeader {
    #[br(assert(magic == *b"BTX0"))]
    pub magic: [u8; 4],
    pub byte_order: u16,
    pub version: u16,
    pub file_size: u32,
    /// Always 0x10
    pub header_size: u16,
    /// Always 1 TEX0 chunk
    pub num_chunks: u16,
    /// Offset from BTX0 to TEX0 Chunk
    pub offset: u32,
}
