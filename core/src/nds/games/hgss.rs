use crate::nds::fs::path::NdsPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HgSsKnownFile {
    /// Map files
    LandData,
    /// Map connections
    MapMatrix,
    /// Internal map names
    MapNames,
    MapTextures,
}

impl HgSsKnownFile {
    pub fn rom_path(&self) -> NdsPath {
        match self {
            HgSsKnownFile::LandData => NdsPath::from("/a/0/6/5"),
            HgSsKnownFile::MapMatrix => NdsPath::from("/a/0/4/1"),
            HgSsKnownFile::MapNames => NdsPath::from("/fielddata/maptable/mapname.bin"),
            HgSsKnownFile::MapTextures => NdsPath::from("/a/0/4/4"),
        }
    }
}

impl From<HgSsKnownFile> for NdsPath {
    fn from(value: HgSsKnownFile) -> Self {
        value.rom_path()
    }
}
