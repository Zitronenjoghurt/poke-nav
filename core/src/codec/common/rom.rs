use crate::codec::common::platform::Platform;
use crate::codec::nds::rom::{NdsRom, NdsRomReadError};
use std::io::{Read, Seek};

pub enum Rom {
    Nds(NdsRom),
}

impl Rom {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError> {
        if NdsRom::probe(reader)? {
            return Ok(Self::Nds(NdsRom::read(reader)?));
        }
        Err(RomReadError::UnknownFormat)
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Nds(rom) => rom.name(),
        }
    }

    pub fn platform(&self) -> Platform {
        match self {
            Self::Nds(_) => Platform::Nds,
        }
    }

    pub fn nds(&self) -> Option<&NdsRom> {
        match self {
            Self::Nds(rom) => Some(rom),
            _ => None,
        }
    }

    pub fn is_nds(&self) -> bool {
        matches!(self, Self::Nds(_))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RomReadError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parse(#[from] binrw::Error),
    #[error("ROM is too small")]
    RomTooSmall,
    #[error("Unknown ROM format")]
    UnknownFormat,
    #[error(transparent)]
    Nds(#[from] NdsRomReadError),
}

pub trait RomTrait: Sized {
    fn probe<R: Read + Seek>(reader: &mut R) -> Result<bool, RomReadError>;
    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError>;
    fn name(&self) -> &str;
}
