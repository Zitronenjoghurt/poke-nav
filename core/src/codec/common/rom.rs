use crate::codec::nds::rom::RawNdsRom;
use std::io::{Read, Seek};

pub enum RawRom {
    Nds(RawNdsRom),
}

impl RawRom {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError> {
        if RawNdsRom::probe(reader)? {
            return Ok(Self::Nds(RawNdsRom::read(reader)?));
        }
        Err(RomReadError::UnknownFormat)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RomReadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] binrw::Error),
    #[error("Unknown ROM format")]
    UnknownFormat,
}

pub trait RawRomTrait: Sized {
    fn probe<R: Read + Seek>(reader: &mut R) -> Result<bool, RomReadError>;
    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError>;
}
