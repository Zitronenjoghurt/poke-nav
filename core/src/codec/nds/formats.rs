use crate::codec::common::rom::RomReadError;
use crate::codec::nds::rom::NdsRomReadError;
use std::io::{Read, Seek};

pub mod hgss_map;
pub mod narc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NdsFileFormat {
    Narc,
}

pub enum ParsedNdsFile {
    Narc(narc::Narc),
}

impl ParsedNdsFile {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError> {
        if narc::Narc::probe(reader)? {
            return Ok(ParsedNdsFile::Narc(narc::Narc::read(reader)?));
        };
        Err(NdsRomReadError::UnknownFileFormat.into())
    }

    pub fn format(&self) -> NdsFileFormat {
        match self {
            ParsedNdsFile::Narc(_) => NdsFileFormat::Narc,
        }
    }
}
