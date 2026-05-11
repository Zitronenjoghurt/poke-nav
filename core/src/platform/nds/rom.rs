use crate::compression::blz::blz_decompress;
use crate::compression::CompressionError;
use crate::platform::nds::fs;
use crate::platform::nds::games::dpp::DppRom;
use crate::platform::nds::games::hgss::HgSsRom;
use crate::platform::nds::games::{NdsGame, NdsGameRom};
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
    pub fn game(&self) -> NdsGame {
        NdsGame::detect(self)
    }

    pub fn game_rom(&'_ self) -> Option<NdsGameRom<'_>> {
        NdsGameRom::try_from(self)
    }

    pub fn dpp_rom(&'_ self) -> Option<DppRom<'_>> {
        DppRom::try_from(self)
    }

    pub fn hgss_rom(&'_ self) -> Option<HgSsRom<'_>> {
        HgSsRom::try_from(self)
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
