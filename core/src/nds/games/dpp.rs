use crate::nds::fs::path::NdsPath;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DppKnownFile {
    /// Map files
    LandData,
    /// Map connections
    MapMatrix,
}

impl DppKnownFile {
    pub fn rom_path(&self) -> NdsPath {
        match self {
            DppKnownFile::LandData => NdsPath::from("/fielddata/land_data/land_data.narc"),
            DppKnownFile::MapMatrix => NdsPath::from("/fielddata/mapmatrix/map_matrix.narc"),
        }
    }
}

impl From<DppKnownFile> for NdsPath {
    fn from(value: DppKnownFile) -> Self {
        value.rom_path()
    }
}
