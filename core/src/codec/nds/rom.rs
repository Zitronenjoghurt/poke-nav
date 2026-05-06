use crate::codec::common::rom::{RawRomTrait, RomReadError};
use binrw::BinRead;
use std::io::{Read, Seek, SeekFrom};

mod header;

pub struct RawNdsRom {
    pub header: header::NdsHeader,
}

impl RawRomTrait for RawNdsRom {
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
        Ok(Self { header })
    }
}
