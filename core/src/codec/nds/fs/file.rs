use crate::codec::nds::formats::hgss_map::HgSsMap;
use crate::codec::nds::formats::narc::Narc;
use crate::codec::nds::formats::{NdsFileFormat, ParsedNdsFile};
use crate::codec::nds::fs::NdsFileSystem;
use std::io::{Cursor, Write};

pub struct NdsFile {
    pub id: u16,
    pub name: String,
    pub parent_dir_id: u16,
    pub size: usize,
    pub data: NdsFileData,
}

impl NdsFile {
    pub fn name_with_ext_fallback(&self) -> String {
        if !self.name.contains('.') {
            let ext = self.data.format().map(|f| f.extension()).unwrap_or("bin");
            format!("{}.{ext}", self.name)
        } else {
            self.name.clone()
        }
    }
}

pub enum NdsFileData {
    Parsed {
        compressed: Vec<u8>,
        parsed: ParsedNdsFile,
    },
    Raw(Vec<u8>),
}

impl NdsFileData {
    pub fn new(data: Vec<u8>) -> Result<Self, std::io::Error> {
        let mut compressed = Vec::new();
        let mut writer = brotli::CompressorWriter::new(&mut compressed, 4096, 4, 22);
        writer.write_all(&data)?;
        drop(writer);

        let mut cursor = Cursor::new(data);
        Ok(match ParsedNdsFile::read(&mut cursor) {
            Ok(parsed) => NdsFileData::Parsed { compressed, parsed },
            Err(_) => NdsFileData::Raw(compressed),
        })
    }

    pub fn raw(&self) -> Result<Vec<u8>, std::io::Error> {
        let data = match self {
            NdsFileData::Parsed { compressed, .. } => compressed,
            NdsFileData::Raw(data) => data,
        };
        let mut decompressed = Vec::new();
        brotli::BrotliDecompress(&mut Cursor::new(data), &mut decompressed)?;
        Ok(decompressed)
    }

    pub fn format(&self) -> Option<NdsFileFormat> {
        match self {
            NdsFileData::Parsed { parsed, .. } => Some(parsed.format()),
            _ => None,
        }
    }

    pub fn nested_fs(&self) -> Option<&NdsFileSystem> {
        match &self {
            NdsFileData::Parsed {
                parsed: ParsedNdsFile::Narc(narc),
                ..
            } => Some(&narc.fs),
            _ => None,
        }
    }

    pub fn narc(&self) -> Option<&Narc> {
        match &self {
            NdsFileData::Parsed {
                parsed: ParsedNdsFile::Narc(narc),
                ..
            } => Some(narc),
            _ => None,
        }
    }

    pub fn gen4map(&self) -> Option<&HgSsMap> {
        match self {
            NdsFileData::Parsed {
                parsed: ParsedNdsFile::HgSsMap(map),
                ..
            } => Some(map),
            _ => None,
        }
    }
}
