use crate::compression::blz::blz_decompress;
use crate::compression::CompressionError;
use crate::nds::fs;
use crate::rom::{RomReadError, RomTrait};
use binrw::BinRead;
use std::io::{Read, Seek, SeekFrom};

pub mod fat;
pub mod fnt;
pub mod header;

pub struct NdsRom {
    pub header: header::NdsHeader,
    pub compressed_arm9_size: Option<usize>,
    pub arm9_binary: Vec<u8>,
    pub arm7_binary: Vec<u8>,
    pub fs: fs::NdsFileSystem,
}

impl RomTrait for NdsRom {
    fn probe<R: Read + Seek>(reader: &mut R) -> Result<bool, RomReadError> {
        let pos = reader.stream_position()?;
        let result: Result<_, std::io::Error> = (|| {
            // Verifies that the rom_header_size field equals 0x4000
            reader.seek(SeekFrom::Start(0x84))?;
            let mut buf = [0u8; 4];
            reader.read_exact(&mut buf)?;
            Ok(u32::from_le_bytes(buf) == 0x4000)
        })();
        reader.seek(SeekFrom::Start(pos))?;
        match result {
            Ok(v) => Ok(v),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, RomReadError> {
        reader.seek(SeekFrom::Start(0))?;
        let header = header::NdsHeader::read(reader)?;

        let mut arm9_binary = vec![0u8; header.arm9_size as usize];
        reader.seek(SeekFrom::Start(header.arm9_offset as u64))?;
        reader
            .read_exact(&mut arm9_binary)
            .map_err(|_| NdsRomReadError::Arm9BinaryLocation)?;
        let compressed_arm9_size = arm9_binary.len();

        let arm9_binary = match blz_decompress(&arm9_binary) {
            Ok(decompressed) => decompressed,
            Err(CompressionError::NotCompressed) => arm9_binary,
            Err(e) => return Err(NdsRomReadError::Arm9Decompression(e).into()),
        };
        let decompressed_arm9_size = arm9_binary.len();

        let mut arm7_binary = vec![0u8; header.arm7_size as usize];
        reader.seek(SeekFrom::Start(header.arm7_offset as u64))?;
        reader
            .read_exact(&mut arm7_binary)
            .map_err(|_| NdsRomReadError::Arm7BinaryLocation)?;

        let fs = fs::NdsFileSystem::read(
            reader,
            header.fat_offset,
            header.fat_size,
            header.fnt_offset,
        )?;

        Ok(Self {
            header,
            compressed_arm9_size: if compressed_arm9_size != decompressed_arm9_size {
                Some(compressed_arm9_size)
            } else {
                None
            },
            arm9_binary,
            arm7_binary,
            fs,
        })
    }

    fn name(&self) -> &str {
        self.header.game_title.as_str()
    }
}

impl NdsRom {
    pub fn find_hgss_map_header_table_offset(&self) -> Option<usize> {
        let pattern: [u8; 10] = [
            0xFF, // wildPokemon = 255
            0x00, // areaDataID = 0
            0x0F, 0x00, // coords packed (unknown0=15, worldmapX=0, worldmapY=0)
            0x00, 0x00, // matrixID = 0
            0x8B, 0x00, // scriptFileID = 139
            0x8F, 0x01, // levelScriptID = 399
        ];

        let offset = self
            .arm9_binary
            .windows(pattern.len())
            .position(|w| w == pattern)?;

        if offset + 48 <= self.arm9_binary.len()
            && self.arm9_binary[offset + 24] == 0xFF
            && self.arm9_binary[offset + 25] == 0x00
        {
            println!("Found HGSS header table at ARM9 offset: 0x{:X}", offset);
            Some(offset)
        } else {
            println!("Pattern found at 0x{:X} but sanity check failed", offset);
            None
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NdsRomReadError {
    #[error("Invalid arm7 binary location")]
    Arm7BinaryLocation,
    #[error("Invalid arm9 binary location")]
    Arm9BinaryLocation,
    #[error("Failed to decompress arm9 binary: {0}")]
    Arm9Decompression(CompressionError),
    #[error("Failed to read the FNT")]
    FNTRead,
    #[error("Unknown file format")]
    UnknownFileFormat,
}
