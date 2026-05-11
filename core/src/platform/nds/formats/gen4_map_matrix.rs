use crate::fmt::format_grid;
use crate::rom::RomReadError;
use binrw::{binrw, BinRead, BinReaderExt};
use std::io::{Read, Seek, SeekFrom};

/// Sources:
/// - https://projectpokemon.org/home/docs/gen-4/map-matrix-r28/
/// - https://github.com/DS-Pokemon-Rom-Editor/DSPRE/blob/27cc51d0429279f1450eccf35a45f8d8f616254d/DS_Map/ROMFiles/GameMatrix.cs
pub struct Gen4MapMatrix {
    pub header: Gen4MapMatrixHeader,
    pub files: Vec<u16>,
    pub headers: Option<Vec<u16>>,
    pub altitudes: Option<Vec<u8>>,
}

impl Gen4MapMatrix {
    pub fn probe<R: Read + Seek>(reader: &mut R) -> Result<bool, RomReadError> {
        let pos = reader.stream_position()?;
        let result: Result<bool, std::io::Error> = (|| {
            let end = reader.seek(SeekFrom::End(0))?;
            reader.seek(SeekFrom::Start(pos))?;
            let total_size = end - pos;

            let mut buf = [0u8; 5];
            reader.read_exact(&mut buf)?;
            let width = buf[0] as u64;
            let height = buf[1] as u64;
            let has_headers = buf[2];
            let has_altitudes = buf[3];
            let prefix_len = buf[4] as u64;

            if width == 0 || height == 0 {
                return Ok(false);
            }

            if width > 64 || height > 64 {
                return Ok(false);
            }

            if has_headers > 1 || has_altitudes > 1 {
                return Ok(false);
            }

            if prefix_len > 0 {
                let mut name_buf = vec![0u8; prefix_len as usize];
                reader.read_exact(&mut name_buf)?;
                if !name_buf.iter().all(|&b| (0x20..=0x7E).contains(&b)) {
                    return Ok(false);
                }
            }

            let bytes_per_cell = 2u64
                + if has_headers == 1 { 2 } else { 0 }
                + if has_altitudes == 1 { 1 } else { 0 };
            let expected = 5 + prefix_len + (width * height * bytes_per_cell);
            Ok(total_size == expected)
        })();
        reader.seek(SeekFrom::Start(pos))?;
        match result {
            Ok(v) => Ok(v),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    pub fn read<R: Read + Seek>(reader: &mut R) -> Result<Self, binrw::Error> {
        let header = Gen4MapMatrixHeader::read(reader)?;
        let count = header.global_map_width as usize * header.global_map_height as usize;

        let headers = if header.headers_section_present == 1 {
            Some(reader.read_le_args::<Vec<u16>>(binrw::VecArgs { count, inner: () })?)
        } else {
            None
        };

        let altitudes = if header.altitudes_section_present == 1 {
            let mut buf = vec![0u8; count];
            reader.read_exact(&mut buf)?;
            Some(buf)
        } else {
            None
        };

        let files: Vec<u16> = reader.read_le_args(binrw::VecArgs { count, inner: () })?;

        Ok(Self {
            header,
            files,
            headers,
            altitudes,
        })
    }

    pub fn format_file_ids(&self) -> String {
        format_grid(&self.files, self.header.global_map_width as usize)
    }

    pub fn format_header_ids(&self) -> Option<String> {
        self.headers
            .as_ref()
            .map(|headers| format_grid(headers, self.header.global_map_width as usize))
    }
}

#[binrw]
#[brw(little)]
pub struct Gen4MapMatrixHeader {
    pub global_map_width: u8,
    pub global_map_height: u8,
    pub headers_section_present: u8,
    pub altitudes_section_present: u8,
    #[bw(calc = prefix_name.len() as u8)]
    pub prefix_name_length: u8,
    #[br(count = prefix_name_length, map = |bytes: Vec<u8>| String::from_utf8_lossy(&bytes).into_owned())]
    #[bw(map = |s: &String| s.as_bytes().to_vec())]
    pub prefix_name: String,
}
