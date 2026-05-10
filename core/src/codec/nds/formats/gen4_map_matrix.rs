use crate::codec::common::rom::RomReadError;
use binrw::{binrw, BinRead, BinReaderExt};
use std::io::{Read, Seek, SeekFrom};

/// Source: https://projectpokemon.org/home/docs/gen-4/map-matrix-r28/
pub struct Gen4MapMatrix {
    pub header: Gen4MapMatrixHeader,
    pub map_indices: Vec<u16>,
    // Speculation, but those layers exist in some files
    // The HG/SS first matrix file called "map" and the one for the safari zone
    pub extra_u16_layer: Option<Vec<u16>>,
    // Speculation
    pub extra_u8_layer: Option<Vec<u8>>,
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
            let prefix_len = buf[4] as u64;

            let bytes_per_cell =
                2u64 + if buf[2] == 1 { 2 } else { 0 } + if buf[3] == 1 { 1 } else { 0 };
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

        let map_indices: Vec<u16> = reader.read_le_args(binrw::VecArgs { count, inner: () })?;

        let extra_u16_layer = if header.has_extra_u16_layer == 1 {
            Some(reader.read_le_args::<Vec<u16>>(binrw::VecArgs { count, inner: () })?)
        } else {
            None
        };

        let extra_u8_layer = if header.has_extra_u8_layer == 1 {
            let mut buf = vec![0u8; count];
            reader.read_exact(&mut buf)?;
            Some(buf)
        } else {
            None
        };

        Ok(Self {
            header,
            map_indices,
            extra_u16_layer,
            extra_u8_layer,
        })
    }

    pub fn format_grid(&self) -> String {
        let w = self.header.global_map_width as usize;
        let h = self.header.global_map_height as usize;

        let col_width = self
            .map_indices
            .iter()
            .map(|v| v.to_string().len())
            .max()
            .unwrap_or(1);

        (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| format!("{:>width$}", self.map_indices[y * w + x], width = col_width))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[binrw]
#[brw(little)]
pub struct Gen4MapMatrixHeader {
    pub global_map_width: u8,
    pub global_map_height: u8,
    // Speculation
    pub has_extra_u16_layer: u8,
    // Speculation
    pub has_extra_u8_layer: u8,
    #[bw(calc = prefix_name.len() as u8)]
    pub prefix_name_length: u8,
    #[br(count = prefix_name_length, map = |bytes: Vec<u8>| String::from_utf8_lossy(&bytes).into_owned())]
    #[bw(map = |s: &String| s.as_bytes().to_vec())]
    pub prefix_name: String,
}
