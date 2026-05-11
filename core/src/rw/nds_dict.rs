use crate::rom::RomReadError;
use crate::rw::zero_padded_string::ZeroPaddedString;
use binrw::BinRead;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};

pub fn read_nds_dict<R, T>(reader: &mut R) -> Result<HashMap<String, T>, RomReadError>
where
    R: Read + Seek,
    T: for<'a> BinRead<Args<'a> = ()>,
{
    let dict_start = reader.stream_position()?;

    let _dummy = u8::read_le(reader)?;
    let entry_count = u8::read_le(reader)?;
    let dict_size = u16::read_le(reader)?;
    let _sub_header_size = u16::read_le(reader)?;
    let unknown_section_size = u16::read_le(reader)?;
    let _constant = u32::read_le(reader)?;

    let trie_remaining = unknown_section_size as i64 - 8;
    reader.seek(SeekFrom::Current(trie_remaining))?;

    let mut data_entries = Vec::with_capacity(entry_count as usize);
    for _ in 0..entry_count {
        data_entries.push(T::read_le(reader)?);
    }

    let names_start = dict_start + dict_size as u64 - 16 * entry_count as u64;
    reader.seek(SeekFrom::Start(names_start))?;

    let mut map = HashMap::with_capacity(entry_count as usize);
    for data in data_entries {
        let name = ZeroPaddedString::<16>::read_le(reader)?;
        map.insert(name.0, data);
    }

    reader.seek(SeekFrom::Start(dict_start + dict_size as u64))?;
    Ok(map)
}
