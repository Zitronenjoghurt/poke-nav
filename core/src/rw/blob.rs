use crate::rom::RomReadError;
use std::io::{Read, Seek, SeekFrom};

pub fn read_blob<R: Read + Seek>(
    reader: &mut R,
    offset: u64,
    len: usize,
) -> Result<Vec<u8>, RomReadError> {
    if len == 0 {
        return Ok(Vec::new());
    }
    reader.seek(SeekFrom::Start(offset))?;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    Ok(buf)
}
