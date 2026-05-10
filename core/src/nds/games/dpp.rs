use crate::nds::fs::path::NdsPath;
use crate::nds::games::{CommonFile, NdsGame};
use crate::nds::rom::NdsRom;

pub struct DppRom<'a> {
    pub rom: &'a NdsRom,
    pub game: DppGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DppGame {
    Diamond,
    Pearl,
    Platinum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DppFile {
    LandData,
    MapMatrix,
}

impl DppFile {
    pub fn path(self) -> NdsPath {
        match self {
            Self::LandData => NdsPath::from("/fielddata/land_data/land_data.narc"),
            Self::MapMatrix => NdsPath::from("/fielddata/mapmatrix/map_matrix.narc"),
        }
    }

    pub fn from_common(file: CommonFile) -> Option<Self> {
        match file {
            CommonFile::LandData => Some(Self::LandData),
            CommonFile::MapMatrix => Some(Self::MapMatrix),
        }
    }
}

impl<'a> DppRom<'a> {
    pub fn try_from(rom: &'a NdsRom) -> Option<Self> {
        let game = match NdsGame::detect(rom) {
            NdsGame::Diamond => DppGame::Diamond,
            NdsGame::Pearl => DppGame::Pearl,
            NdsGame::Platinum => DppGame::Platinum,
            _ => return None,
        };
        Some(Self { rom, game })
    }
}
