use binrw::binrw;

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
/// Source: https://problemkaputt.de/gbatek-ds-cartridge-nitrorom-and-nitroarc-file-systems.htm
pub struct FntMainEntry {
    pub sub_table_offset: u32,
    pub first_file_id: u16,
    pub parent_id_or_count: u16,
}

#[binrw]
#[brw(little)]
#[derive(Debug, Clone)]
/// Source: https://problemkaputt.de/gbatek-ds-cartridge-nitrorom-and-nitroarc-file-systems.htm
pub struct FntSubEntry {
    pub type_length: u8,
    // The length is the bottom 7 bits of the type_length byte.
    // If type_length is 0, this safely reads 0 bytes!
    #[br(count = type_length & 0x7F)]
    pub name_bytes: Vec<u8>,
    // Only directories (bit 7 set) have a 16-bit ID attached
    #[br(if(type_length >= 0x80))]
    pub sub_dir_id: Option<u16>,
}

impl FntSubEntry {
    pub fn is_end(&self) -> bool {
        self.type_length == 0x00
    }

    pub fn is_dir(&self) -> bool {
        self.type_length >= 0x80
    }

    pub fn name(&self) -> String {
        String::from_utf8_lossy(&self.name_bytes).into_owned()
    }
}
