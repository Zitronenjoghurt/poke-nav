use binrw::binrw;

#[binrw]
#[brw(little)]
#[br(import(fat_size: u32))]
/// Source: https://problemkaputt.de/gbatek-ds-cartridge-nitrorom-and-nitroarc-file-systems.htm
pub struct Fat {
    #[br(count = fat_size / 8)]
    pub entries: Vec<FatEntry>,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
pub struct FatEntry {
    /// Start address (relative to IMG base, which is 0 for the ROM)
    pub start_address: u32,
    /// End address (Start + Length)
    pub end_address: u32,
}

impl FatEntry {
    pub fn is_unused(&self) -> bool {
        self.start_address == 0 && self.end_address == 0
    }

    pub fn size(&self) -> u32 {
        self.end_address.saturating_sub(self.start_address)
    }
}
