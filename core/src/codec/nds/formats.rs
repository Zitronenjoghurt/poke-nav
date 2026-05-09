use crate::codec::common::rom::RomReadError;
use crate::codec::nds::rom::NdsRomReadError;
use std::io::{Read, Seek};

pub mod hgss_map;
pub mod narc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum NdsFileFormat {
    Narc,
    HgSsMap,
}

impl NdsFileFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            NdsFileFormat::Narc => "narc",
            NdsFileFormat::HgSsMap => "hgssmap",
        }
    }

    pub fn full_name(&self) -> &'static str {
        match self {
            NdsFileFormat::Narc => "Nitro Archive Container",
            NdsFileFormat::HgSsMap => "HeartGold/SoulSilver Map Data",
        }
    }

    pub fn explanation(&self) -> &'static str {
        match self {
            NdsFileFormat::Narc => {
                "A Nitro Archive containing multiple sub-files, used to bundle related assets together."
            }
            NdsFileFormat::HgSsMap => {
                "A map file containing tile permissions, 3D objects, an NSBMD model, and terrain data."
            }
        }
    }
}

pub enum ParsedNdsFile {
    Narc(narc::Narc),
    HgSsMap(hgss_map::HgSsMap),
}

impl ParsedNdsFile {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError> {
        if narc::Narc::probe(reader)? {
            return Ok(ParsedNdsFile::Narc(narc::Narc::read(reader)?));
        };
        if hgss_map::HgSsMap::probe(reader)? {
            return Ok(ParsedNdsFile::HgSsMap(hgss_map::HgSsMap::read(reader)?));
        }
        Err(NdsRomReadError::UnknownFileFormat.into())
    }

    pub fn format(&self) -> NdsFileFormat {
        match self {
            ParsedNdsFile::Narc(_) => NdsFileFormat::Narc,
            ParsedNdsFile::HgSsMap(_) => NdsFileFormat::HgSsMap,
        }
    }
}
