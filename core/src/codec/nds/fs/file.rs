use crate::codec::nds::formats::ParsedNdsFile;

pub struct NdsFile {
    pub id: u16,
    pub name: String,
    pub parent_dir_id: u16,
    pub size: usize,
    pub data: NdsFileData,
}

pub enum NdsFileData {
    Parsed(ParsedNdsFile),
    Raw(Vec<u8>),
}

impl NdsFileData {
    pub fn raw(&self) -> Option<&[u8]> {
        match self {
            NdsFileData::Raw(data) => Some(data),
            _ => None,
        }
    }
}
