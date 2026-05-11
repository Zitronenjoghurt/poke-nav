use crate::platform::nds::rom::NdsRomReadError;
use crate::rom::RomReadError;
use std::fmt::Display;
use std::io::{Read, Seek};

pub mod gen4_map_data;
pub mod gen4_map_matrix;
pub mod hgss_map_header;
pub mod narc;
pub mod nsbtx;
pub mod nstex;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "strum", derive(strum::EnumIter))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NdsFileFormat {
    Narc,
    Nsbtx,
    Nstex,
    Gen4MapData,
    Gen4MapMatrix,
}

impl NdsFileFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            NdsFileFormat::Narc => "narc",
            NdsFileFormat::Nsbtx => "nsbtx",
            NdsFileFormat::Nstex => "nstex",
            NdsFileFormat::Gen4MapData => "gen4mapdat",
            NdsFileFormat::Gen4MapMatrix => "gen4mapmat",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            NdsFileFormat::Narc => "NARC",
            NdsFileFormat::Nsbtx => "NSBTX",
            NdsFileFormat::Nstex => "NSTEX",
            NdsFileFormat::Gen4MapData => "G4MAPDAT",
            NdsFileFormat::Gen4MapMatrix => "G4MAPMAT",
        }
    }

    pub fn full_name(&self) -> &'static str {
        match self {
            NdsFileFormat::Nsbtx => "Nitro System Binary Texture",
            NdsFileFormat::Nstex => "Nitro System Texture Data",
            NdsFileFormat::Narc => "Nitro Archive Container",
            NdsFileFormat::Gen4MapData => "Pokémon Generation 4 Map Data",
            NdsFileFormat::Gen4MapMatrix => "Pokémon Generation 4 Map Matrix",
        }
    }

    pub fn explanation(&self) -> &'static str {
        match self {
            NdsFileFormat::Nsbtx => "A texture file containing a texture data chunk.",
            NdsFileFormat::Nstex => "A texture file which stores a set of 3D textures.",
            NdsFileFormat::Narc => {
                "A Nitro Archive containing multiple sub-files, used to bundle related assets together."
            }
            NdsFileFormat::Gen4MapData => {
                "A map file containing tile permissions, 3D objects, an NSBMD model, and terrain data."
            }
            NdsFileFormat::Gen4MapMatrix => {
                "A map file for associating the rendered map to the map data."
            }
        }
    }
}

impl Display for NdsFileFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.short_name())
    }
}

pub enum ParsedNdsFile {
    Narc(narc::Narc),
    Nsbtx(nsbtx::Nsbtx),
    Nstex(nstex::Nstex),
    Gen4MapData(gen4_map_data::Gen4MapData),
    Gen4MapMatrix(gen4_map_matrix::Gen4MapMatrix),
}

impl ParsedNdsFile {
    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError> {
        if narc::Narc::probe(reader)? {
            return Ok(ParsedNdsFile::Narc(narc::Narc::read(reader)?));
        };
        if gen4_map_data::Gen4MapData::probe(reader)? {
            return Ok(ParsedNdsFile::Gen4MapData(
                gen4_map_data::Gen4MapData::read(reader)?,
            ));
        }
        if gen4_map_matrix::Gen4MapMatrix::probe(reader)? {
            return Ok(ParsedNdsFile::Gen4MapMatrix(
                gen4_map_matrix::Gen4MapMatrix::read(reader)?,
            ));
        }
        if nsbtx::Nsbtx::probe(reader)? {
            return Ok(ParsedNdsFile::Nsbtx(nsbtx::Nsbtx::read(reader)?));
        }
        if nstex::Nstex::probe(reader)? {
            return Ok(ParsedNdsFile::Nstex(nstex::Nstex::read(reader)?));
        }
        Err(NdsRomReadError::UnknownFileFormat.into())
    }

    pub fn format(&self) -> NdsFileFormat {
        match self {
            ParsedNdsFile::Narc(_) => NdsFileFormat::Narc,
            ParsedNdsFile::Nsbtx(_) => NdsFileFormat::Nsbtx,
            ParsedNdsFile::Nstex(_) => NdsFileFormat::Nstex,
            ParsedNdsFile::Gen4MapData(_) => NdsFileFormat::Gen4MapData,
            ParsedNdsFile::Gen4MapMatrix(_) => NdsFileFormat::Gen4MapMatrix,
        }
    }
}
