use crate::platform::nds::fs::path::NdsPath;
use crate::platform::nds::games::dpp::{DppFile, DppRom};
use crate::platform::nds::games::hgss::{HgSsFile, HgSsRom};
use crate::platform::nds::rom::NdsRom;

pub mod dpp;
pub mod hgss;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum NdsGame {
    Diamond,
    Pearl,
    Platinum,
    HeartGold,
    SoulSilver,
    Unknown,
}

impl NdsGame {
    pub fn detect(rom: &NdsRom) -> Self {
        // Could eventually detect the game's region by taking the 4th byte into account
        match &rom.header.game_code.as_bytes()[..3] {
            b"ADA" => Self::Diamond,
            b"APA" => Self::Pearl,
            b"CPU" => Self::Platinum,
            b"IPK" => Self::HeartGold,
            b"IPG" => Self::SoulSilver,
            _ => Self::Unknown,
        }
    }

    pub fn is_dpp(&self) -> bool {
        matches!(self, Self::Diamond | Self::Pearl | Self::Platinum)
    }

    pub fn is_hgss(&self) -> bool {
        matches!(self, Self::HeartGold | Self::SoulSilver)
    }
}

pub enum NdsGameRom<'a> {
    Dpp(DppRom<'a>),
    HgSs(HgSsRom<'a>),
}

impl<'a> NdsGameRom<'a> {
    pub fn try_from(rom: &'a NdsRom) -> Option<Self> {
        if let Some(dpp) = DppRom::try_from(rom) {
            return Some(Self::Dpp(dpp));
        }
        if let Some(hgss) = HgSsRom::try_from(rom) {
            return Some(Self::HgSs(hgss));
        }
        None
    }

    pub fn resolve(&self, file: CommonFile) -> Option<NdsPath> {
        match self {
            Self::Dpp(_) => DppFile::from_common(file).map(|f| f.path()),
            Self::HgSs(_) => HgSsFile::from_common(file).map(|f| f.path()),
        }
    }

    pub fn rom(&self) -> &NdsRom {
        match self {
            Self::Dpp(r) => r.rom,
            Self::HgSs(r) => r.rom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommonFile {
    LandData,
    MapMatrix,
}
