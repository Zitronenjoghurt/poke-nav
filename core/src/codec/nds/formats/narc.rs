use crate::codec::common::rom::RomReadError;
use crate::codec::nds::fs::NdsFileSystem;
use crate::codec::nds::rom::fat::FatEntry;
use binrw::{binrw, BinRead};
use std::io::{Read, Seek, SeekFrom};

pub struct Narc {
    pub header: NarcHeader,
    pub btaf: BtafChunk,
    pub btnf: BtnfChunkHeader,
    pub gmif: GmifChunkHeader,
    pub fs: NdsFileSystem,
}

impl Narc {
    pub fn probe<R: Read + Seek>(reader: &mut R) -> Result<bool, RomReadError> {
        let pos = reader.stream_position()?;
        let mut magic = [0u8; 4];
        let ok = reader.read_exact(&mut magic).is_ok() && magic == *b"NARC";
        reader.seek(SeekFrom::Start(pos))?;
        Ok(ok)
    }

    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError> {
        let header = NarcHeader::read(reader)?;
        let btaf = BtafChunk::read(reader)?;

        let btnf_start = reader.stream_position()?;
        let btnf = BtnfChunkHeader::read(reader)?;
        let fnt_base = reader.stream_position()?;

        let btnf_end_aligned = (btnf_start + btnf.chunk_size as u64 + 3) & !3;
        reader.seek(SeekFrom::Start(btnf_end_aligned))?;

        let gmif = GmifChunkHeader::read(reader)?;
        let img_base = reader.stream_position()?;

        let fs = NdsFileSystem::read_tables(reader, &btaf.entries, fnt_base, img_base)?;

        Ok(Self {
            header,
            btaf,
            btnf,
            gmif,
            fs,
        })
    }
}

#[binrw]
#[brw(little)]
pub struct NarcHeader {
    #[br(assert(magic == *b"NARC"))]
    pub magic: [u8; 4],
    pub byte_order: u16,
    pub version: u16,
    pub file_size: u32,
    pub chunk_size: u16,
    pub num_chunks: u16,
}

#[binrw]
#[brw(little)]
pub struct BtafChunk {
    #[br(assert(magic == *b"BTAF"))]
    pub magic: [u8; 4],
    pub chunk_size: u32,
    pub num_files: u16,
    pub reserved: u16,
    #[br(count = num_files)]
    pub entries: Vec<FatEntry>,
}

#[binrw]
#[brw(little)]
pub struct BtnfChunkHeader {
    #[br(assert(magic == *b"BTNF"))]
    pub magic: [u8; 4],
    pub chunk_size: u32,
}

#[binrw]
#[brw(little)]
pub struct GmifChunkHeader {
    #[br(assert(magic == *b"GMIF"))]
    pub magic: [u8; 4],
    pub chunk_size: u32,
}
