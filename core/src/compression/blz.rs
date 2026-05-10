use crate::compression::CompressionError;
use binrw::{binrw, BinReaderExt};
use std::io::{Cursor, Seek, SeekFrom};

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct BlzFooter {
    /// Packed: low 24 bits = total compressed size, high 8 bits = header size.
    pub packed_sizes: u32,
    /// How many extra bytes the decompressed output has over the input.
    pub extra_size: u32,
}

impl BlzFooter {
    pub fn total_size(&self) -> usize {
        (self.packed_sizes & 0x00FF_FFFF) as usize
    }

    pub fn hdr_size(&self) -> usize {
        (self.packed_sizes >> 24) as usize
    }
}

/// Decompress a BLZ (Backwards LZ) compressed NDS ARM binary.
///
/// Returns `Err(NotCompressed)` if the footer indicates no compression.
///
/// Source: https://github.com/simontime/arm9dec
pub fn blz_decompress(data: &[u8]) -> Result<Vec<u8>, CompressionError> {
    let len = data.len();
    if len < 8 {
        return Err(CompressionError::NotCompressed);
    }

    let mut cursor = Cursor::new(data);
    cursor.seek(SeekFrom::End(-8))?;
    let footer: BlzFooter = cursor.read_le()?;

    let total_size = footer.total_size();
    let hdr_size = footer.hdr_size();
    let extra_size = footer.extra_size as usize;

    if extra_size == 0 || hdr_size < 8 || total_size > len || hdr_size > total_size {
        return Err(CompressionError::NotCompressed);
    }

    let decompressed_len = len + extra_size;
    let comp_start = len - total_size;

    let mut out = vec![0u8; decompressed_len];
    out[..comp_start].copy_from_slice(&data[..comp_start]);

    let mut src = len - hdr_size;
    let mut dst = decompressed_len;

    while src > comp_start && dst > comp_start {
        src -= 1;
        let flags = data[src];

        for bit in 0..8u8 {
            if src <= comp_start || dst <= comp_start {
                break;
            }

            if flags & (0x80 >> bit) != 0 {
                if src < comp_start + 2 {
                    return Err(CompressionError::BlzOutOfBounds { src, dst });
                }
                src -= 2;
                let op = (data[src + 1] as usize) << 8 | data[src] as usize;
                let run = (op >> 12) + 3;
                let ofs = (op & 0xFFF) + 3;

                for _ in 0..run {
                    if dst == 0 || dst + ofs > decompressed_len {
                        return Err(CompressionError::BlzOutOfBounds { src, dst });
                    }
                    dst -= 1;
                    out[dst] = out[dst + ofs];
                }
            } else {
                src -= 1;
                dst -= 1;
                out[dst] = data[src];
            }
        }
    }

    Ok(out)
}
