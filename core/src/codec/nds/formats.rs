use crate::codec::common::rom::RomReadError;
use crate::codec::nds::rom::NdsRomReadError;
use std::io::{Read, Seek};

pub mod gen4_map;
pub mod narc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NdsFileFormat {
    Narc,
    Gen4Map,
}

impl NdsFileFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            NdsFileFormat::Narc => "narc",
            NdsFileFormat::Gen4Map => "gen4map",
        }
    }
}

pub enum ParsedNdsFile {
    Narc(narc::Narc),
    Gen4Map(gen4_map::Gen4Map),
}

impl ParsedNdsFile {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError> {
        if narc::Narc::probe(reader)? {
            return Ok(ParsedNdsFile::Narc(narc::Narc::read(reader)?));
        };
        if gen4_map::Gen4Map::probe(reader)? {
            return Ok(ParsedNdsFile::Gen4Map(gen4_map::Gen4Map::read(reader)?));
        }
        Err(NdsRomReadError::UnknownFileFormat.into())
    }

    pub fn format(&self) -> NdsFileFormat {
        match self {
            ParsedNdsFile::Narc(_) => NdsFileFormat::Narc,
            ParsedNdsFile::Gen4Map(_) => NdsFileFormat::Gen4Map,
        }
    }
}
