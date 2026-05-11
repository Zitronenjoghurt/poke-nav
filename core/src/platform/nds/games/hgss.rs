use crate::platform::nds::formats::hgss_map_header::HGSSMapHeader;
use crate::platform::nds::fs::path::NdsPath;
use crate::platform::nds::games::{CommonFile, NdsGame};
use crate::platform::nds::rom::NdsRom;
use binrw::BinRead;
use std::io::Cursor;

pub struct HgSsRom<'a> {
    pub rom: &'a NdsRom,
    pub game: HgSsGame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HgSsGame {
    HeartGold,
    SoulSilver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HgSsFile {
    LandData,
    MapMatrix,
    MapNames,
    MapTextures,
}

impl HgSsFile {
    pub fn path(self) -> NdsPath {
        match self {
            Self::LandData => NdsPath::from("/a/0/6/5"),
            Self::MapMatrix => NdsPath::from("/a/0/4/1"),
            Self::MapNames => NdsPath::from("/fielddata/maptable/mapname.bin"),
            Self::MapTextures => NdsPath::from("/a/0/4/4"),
        }
    }

    pub fn from_common(file: CommonFile) -> Option<Self> {
        match file {
            CommonFile::LandData => Some(Self::LandData),
            CommonFile::MapMatrix => Some(Self::MapMatrix),
        }
    }
}

impl<'a> HgSsRom<'a> {
    pub const MAP_HEADER_TABLE_LENGTH: usize = 540;

    pub fn try_from(rom: &'a NdsRom) -> Option<Self> {
        let game = match NdsGame::detect(rom) {
            NdsGame::HeartGold => HgSsGame::HeartGold,
            NdsGame::SoulSilver => HgSsGame::SoulSilver,
            _ => return None,
        };
        Some(Self { rom, game })
    }

    pub fn find_map_header_table_offset(&self) -> Option<usize> {
        let pattern: [u8; 10] = [
            0xFF, // wildPokemon = 255
            0x00, // areaDataID = 0
            0x0F, 0x00, // coords packed (unknown0=15, worldmapX=0, worldmapY=0)
            0x00, 0x00, // matrixID = 0
            0x8B, 0x00, // scriptFileID = 139
            0x8F, 0x01, // levelScriptID = 399
        ];

        let offset = self
            .rom
            .arm9_binary
            .windows(pattern.len())
            .position(|w| w == pattern)?;

        if offset + 48 <= self.rom.arm9_binary.len()
            && self.rom.arm9_binary[offset + 24] == 0xFF
            && self.rom.arm9_binary[offset + 25] == 0x00
        {
            Some(offset)
        } else {
            None
        }
    }

    pub fn read_map_headers(&self) -> Vec<HGSSMapHeader> {
        let Some(offset) = self.find_map_header_table_offset() else {
            return vec![];
        };
        let data = &self.rom.arm9_binary[offset..];
        let mut cursor = Cursor::new(data);
        (0..Self::MAP_HEADER_TABLE_LENGTH)
            .map_while(|_| HGSSMapHeader::read_le(&mut cursor).ok())
            .collect()
    }
}
