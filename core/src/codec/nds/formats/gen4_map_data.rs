use crate::codec::common::rom::RomReadError;
use binrw::{binrw, BinRead, BinReaderExt};
use std::io::{Read, Seek, SeekFrom};

/// Sources:
/// - https://github.com/DS-Pokemon-Rom-Editor/DSPRE
/// - https://hirotdk.neocities.org/FileSpecs#Maps
/// - https://projectpokemon.org/home/docs/gen-4/map-structure-r29/
pub struct Gen4MapData {
    pub header: Gen4MapDataHeader,
    /// HG/SS-only
    pub background_sound_section: Option<Vec<u8>>,
    pub permissions: Gen4MapPermissions,
    pub objects: Vec<Gen4MapObject>,
    pub nsbmd: Vec<u8>,
    pub bdhc: Vec<u8>,
}

impl Gen4MapData {
    pub fn probe<R: Read + Seek>(reader: &mut R) -> Result<bool, RomReadError> {
        let pos = reader.stream_position()?;
        let result: Result<bool, std::io::Error> = (|| {
            let mut buf = [0u8; 8];
            reader.read_exact(&mut buf)?;
            let perm_size = u32::from_le_bytes(buf[0..4].try_into().unwrap());
            let obj_size = u32::from_le_bytes(buf[4..8].try_into().unwrap());
            Ok(perm_size == 0x800 && obj_size % 48 == 0)
        })();
        reader.seek(SeekFrom::Start(pos))?;
        match result {
            Ok(v) => Ok(v),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, binrw::Error> {
        let header = Gen4MapDataHeader::read(reader)?;

        // HG/SS-specific background sound magic
        let background_sound_section_magic: u16 = reader.read_le()?;
        let background_sound_section = if background_sound_section_magic == 0x1234 {
            let size: u16 = reader.read_le()?;
            let mut buf = vec![0u8; size as usize];
            reader.read_exact(&mut buf)?;
            Some(buf)
        } else {
            reader.seek(SeekFrom::Current(-2))?;
            None
        };

        let permissions = Gen4MapPermissions::read(reader)?;

        let num_objects = header.objects_size / 48;
        let objects: Vec<Gen4MapObject> = reader.read_le_args(binrw::VecArgs {
            count: num_objects as usize,
            inner: (),
        })?;

        let mut nsbmd = vec![0u8; header.nsbmd_size as usize];
        reader.read_exact(&mut nsbmd)?;

        let mut bdhc = vec![0u8; header.bdhc_size as usize];
        reader.read_exact(&mut bdhc)?;

        Ok(Self {
            header,
            background_sound_section,
            permissions,
            objects,
            nsbmd,
            bdhc,
        })
    }
}

#[binrw]
#[brw(little)]
pub struct Gen4MapDataHeader {
    /// Always 0x800
    pub permission_size: u32,
    pub objects_size: u32,
    pub nsbmd_size: u32,
    pub bdhc_size: u32,
}

#[binrw]
#[brw(little)]
pub struct Gen4MapPermissions {
    /// 32×32 grid, ordered left-to-right, top-to-bottom
    #[br(count = 32 * 32)]
    pub tiles: Vec<Gen4TilePermission>,
}

impl Gen4MapPermissions {
    pub fn format_grid(&self) -> String {
        let mut out = String::new();
        for row in 0..32 {
            for col in 0..32 {
                let tile = &self.tiles[row * 32 + col];
                let ch = if tile.collision() == Gen4MapCollision::Blocked {
                    "██"
                } else {
                    tile.special_behavior().icon()
                };
                out.push_str(ch);
            }
            out.push('\n');
        }
        out
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Gen4TilePermission {
    /// First Byte: Special behavior — surfable, tall grass, ledge, warp, etc.
    pub special: u8,
    /// Second Byte: Collision and terrain sound/effect.
    pub movement: u8,
}

impl Gen4TilePermission {
    pub fn special_behavior(&self) -> Gen4MapSpecialPermission {
        Gen4MapSpecialPermission::from(self.special)
    }

    pub fn collision(&self) -> Gen4MapCollision {
        Gen4MapCollision::from(self.movement)
    }
}

/// Source: https://github.com/DS-Pokemon-Rom-Editor/DSPRE/blob/27cc51d0429279f1450eccf35a45f8d8f616254d/DS_Map/Resources/PokeDatabase.cs#L653
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gen4MapCollision {
    Walkable,     // 0x00
    Snow,         // 0x01
    Leaves,       // 0x02
    Grass,        // 0x04
    StairsAndIce, // 0x06
    Metal,        // 0x07
    CavernFloor,  // 0x0A
    Wood,         // 0x0D
    Blocked,      // 0x80
    Unknown(u8),
}

impl From<u8> for Gen4MapCollision {
    fn from(val: u8) -> Self {
        match val {
            0x00 => Gen4MapCollision::Walkable,
            0x01 => Gen4MapCollision::Snow,
            0x02 => Gen4MapCollision::Leaves,
            0x04 => Gen4MapCollision::Grass,
            0x06 => Gen4MapCollision::StairsAndIce,
            0x07 => Gen4MapCollision::Metal,
            0x0A => Gen4MapCollision::CavernFloor,
            0x0D => Gen4MapCollision::Wood,
            0x80 => Gen4MapCollision::Blocked,
            other => Gen4MapCollision::Unknown(other),
        }
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct Gen4MapObject {
    pub object_id: u32,     // 0x00,
    pub y_frac: u16,        // 0x04
    pub y_coord: u16,       // 0x06
    pub z_frac: u16,        // 0x08
    pub z_coord: u16,       // 0x0A
    pub x_frac: u16,        // 0x0C
    pub x_coord: u16,       // 0x0E
    pub unknown1: [u8; 13], // 0x10–0x1C
    pub width: u32,         // 0x1D
    pub height: u32,        // 0x21
    pub length: u32,        // 0x25
    pub unknown2: [u8; 7],  // 0x29–0x2F
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gen4MapSpecialPermission {
    FreePassage,
    Grass,
    HighGrass,
    CaveEncounter,
    PlainsEncounter,
    Surfing,
    Waterfall,
    GoDown,
    WaterSplash,
    FootOnSand,
    Blocked,
    JumpUp,
    JumpDown,
    RotateRight,
    RotateLeft,
    RotateUp,
    RotateDown,
    RockClimb,
    StairsDown,
    StairsUp,
    DoorNoAnim,
    ExitBuilding,
    DoorJumpWarp,
    DoorOpening,
    ForceBike,
    OpenPc,
    OpenMapSinnoh,
    BattleWatch,
    SnowWaist,
    SnowHead,
    MudWaist,
    MudHead,
    GrassMud,
    UnderGrassMud,
    SnowLow,
    RideJumpLeft,
    RideBike,
    SignBookshelf,
    SignTrashCan,
    SignShelf,
    SignNothing,
    Unknown(u8),
}

impl Gen4MapSpecialPermission {
    pub fn icon(&self) -> &'static str {
        use Gen4MapSpecialPermission::*;
        match self {
            FreePassage => "  ",
            Grass | HighGrass => ",,",
            GrassMud | UnderGrassMud => ",~",
            CaveEncounter => "..",
            PlainsEncounter => ". ",
            Surfing => "~~",
            Waterfall => "~v",
            WaterSplash => "~*",
            GoDown => "vv",
            Blocked => "XX",
            StairsUp => "/^",
            StairsDown => "/v",
            JumpUp => "^^",
            JumpDown => "vv",
            RotateRight => " >",
            RotateLeft => "< ",
            RotateUp => " ^",
            RotateDown => " v",
            RockClimb => "RC",
            DoorNoAnim | DoorOpening => "[]",
            DoorJumpWarp => "[!",
            ExitBuilding => "][",
            ForceBike | RideBike => "BB",
            OpenPc => "PC",
            OpenMapSinnoh => "MP",
            BattleWatch => "BW",
            SnowWaist => "::",
            SnowHead => ":^",
            SnowLow => ":.",
            MudWaist => "%%",
            MudHead => "%^",
            FootOnSand => "°°",
            RideJumpLeft => "<^",
            SignBookshelf => "SB",
            SignTrashCan => "ST",
            SignShelf => "SS",
            SignNothing => "S?",
            Unknown(_) => "??",
        }
    }
}

impl From<u8> for Gen4MapSpecialPermission {
    fn from(val: u8) -> Self {
        use Gen4MapSpecialPermission::*;
        match val {
            0x00
            | 0x04..=0x09
            | 0x0C..=0x0F
            | 0x18
            | 0x1A..=0x1B
            | 0x1D..=0x20
            | 0x23..=0x29
            | 0x2B..=0x32
            | 0x34..=0x35
            | 0x38..=0x39
            | 0x3C..=0x3F
            | 0x44..=0x48
            | 0x4A
            | 0x4C..=0x4F
            | 0x54..=0x5D
            | 0x60..=0x63
            | 0x66
            | 0x68
            | 0x6A..=0x6D
            | 0x6F..=0x70
            | 0x72
            | 0x74..=0x77
            | 0x79..=0x7B
            | 0x7D..=0x82
            | 0x84
            | 0x87..=0xA0
            | 0xAA..=0xD2
            | 0xDC..=0xDF
            | 0xED..=0xFF => FreePassage,
            0x02 => Grass,
            0x03 => HighGrass,
            0x0A => CaveEncounter,
            0x0B => PlainsEncounter,
            0x10..=0x12 | 0x14..=0x15 | 0x19 | 0x22 | 0x2A | 0x50..=0x53 | 0x73 | 0x78 | 0x7C => {
                Surfing
            }
            0x13 => Waterfall,
            0x16 | 0x1C => GoDown,
            0x17 => WaterSplash,
            0x21 => FootOnSand,
            0x33 | 0x36..=0x37 | 0x49 => Blocked,
            0x3A => JumpUp,
            0x3B => JumpDown,
            0x40 => RotateRight,
            0x41 => RotateLeft,
            0x42 => RotateUp,
            0x43 => RotateDown,
            0x4B => RockClimb,
            0x5E => StairsDown,
            0x5F => StairsUp,
            0x64 | 0x6E => DoorNoAnim,
            0x65 => ExitBuilding,
            0x67 => DoorJumpWarp,
            0x69 => DoorOpening,
            0x71 | 0xDB => ForceBike,
            0x83 => OpenPc,
            0x85 => OpenMapSinnoh,
            0x86 => BattleWatch,
            0xA1 => SnowWaist,
            0xA3 => SnowHead,
            0xA4 => MudWaist,
            0xA5 => MudHead,
            0xA6 => GrassMud,
            0xA7 => UnderGrassMud,
            0xA9 => SnowLow,
            0xD3..=0xD9 => RideJumpLeft,
            0xDA => RideBike,
            0xE0..=0xE2 | 0xEB..=0xEC => SignBookshelf,
            0xE4 => SignTrashCan,
            0xE5 => SignShelf,
            0xE6..=0xEA => SignNothing,
            0xE3 => FreePassage,
            other => Unknown(other),
        }
    }
}
