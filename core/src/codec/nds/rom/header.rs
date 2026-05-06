use crate::codec::common::rw::zero_padded_string::ZeroPaddedString;
use binrw::binrw;

#[binrw]
#[brw(little)]
/// Source: https://problemkaputt.de/gbatek-ds-cartridge-header.htm
pub struct NdsHeader {
    pub game_title: ZeroPaddedString<12>,
    /// NTR-xxxx, 0 is referring to homebrew
    pub game_code: ZeroPaddedString<4>,
    pub maker_code: NdsMakerCode,
    pub unit_code: NdsUnitCode,
    /// 0x00..=0x07, usually 0x00
    pub encryption_seed_select: u8,
    /// Chip size = 128KB SHL nn (eg. 7 = 16MB)
    pub device_capacity: u8,
    pub _reserved_1: [u8; 7],
    pub dsi_flags: u8,
    pub nds_region_or_dsi_permit: u8,
    pub rom_version: u8,
    /// Bit 2: Skip "Press Button" after Health and Safety
    pub autostart: u8,
    /// 0x4000 and up, aligned by 0x1000
    pub arm9_offset: u32,
    /// 0x2000000..=0x23BFE00
    pub arm9_entry_address: u32,
    /// 0x2000000..=0x23BFE00
    pub arm9_ram_address: u32,
    /// max. 0x3BFE00 (3839.5KB)
    pub arm9_size: u32,
    /// 0x8000 and up
    pub arm7_offset: u32,
    /// 0x2000000..=0x23BFE00, or 0x37F8000..=0x3807E00
    pub arm7_entry_address: u32,
    /// 0x2000000..=0x23BFE00, or 0x37F8000..=0x3807E00
    pub arm7_ram_address: u32,
    /// max. 0x3BFE00 (3839.5KB), or 0xFE00 (63.5KB)
    pub arm7_size: u32,
    pub fnt_offset: u32,
    pub fnt_size: u32,
    pub fat_offset: u32,
    pub fat_size: u32,
    pub arm9_overlay_offset: u32,
    pub arm9_overlay_size: u32,
    pub arm7_overlay_offset: u32,
    pub arm7_overlay_size: u32,
    /// ROMCTRL (0x40001A4h) setting for normal (unencrypted) commands
    pub normal_romctrl: u32,
    /// ROMCTRL (0x40001A4h) setting for KEY1 (encrypted) commands
    pub key1_romctrl: u32,
    /// 0x0 = None, 0x8000 and up
    pub icon_title_offset: u32,
    /// CRC-16 of [[0x020]..=0x00007FFF]
    pub secure_area_checksum: u16,
    /// In 131 kHz units (0x051E=10ms, 0x0D7E=26ms)
    pub secure_area_delay: u16,
    /// End address of ARM9 auto-load function list in RAM
    pub arm9_auto_load_hook_addr: u32,
    /// End address of ARM7 auto-load function list in RAM
    pub arm7_auto_load_hook_addr: u32,
    /// Secure area is disabled if this contains "NmMdOnly" encrypted, is usually zero
    pub secure_area_disable: [u8; 8],
    /// Total used ROM size (bytes after this are 0xFF padding, excluding DSi area)
    pub total_used_rom_size: u32,
    /// 0x4000
    pub rom_header_size: u32,
    /// 0x088..=0x093
    pub _reserved_2: [u8; 12],
    /// NAND end of ROM area (in 0x20000-byte units, DSi: 0x80000-byte)
    pub nand_rom_end: u16,
    /// NAND start of read-write area (in 0x20000-byte units, 0 = none)
    pub nand_rw_start: u16,
    pub _reserved_3: [u8; 0x28],
    pub nintendo_logo: [u8; 0x9C],
    /// CRC-16 of [0x0C0-0x15B], fixed 0xCF56
    pub nintendo_logo_checksum: u16,
    /// CRC of [0x000-0x15D]
    pub header_checksum: u16,
    /// 0x0 = None, 0x8000 and up
    pub debug_rom_offset: u32,
    /// 0x0 = None, max. 0x3BFE00
    pub debug_size: u32,
    /// 0x0 = None, 0x2400000..=0x27BFE00
    pub debug_ram_address: u32,
    pub _reserved_4: [u8; 4],
    pub _reserved_5: [u8; 0x90],
    pub _reserved_6: [u8; 0xE00],
}

impl NdsHeader {
    pub fn chip_size(&self) -> usize {
        (128 * 1024) << self.device_capacity as usize
    }

    pub fn is_skip_press_button(&self) -> bool {
        self.autostart & 0x04 != 0
    }
}

#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NdsMakerCode {
    #[brw(magic = b"00")]
    Homebrew,
    #[brw(magic = b"01")]
    Nintendo,
    Unknown([u8; 2]),
}

#[binrw]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NdsUnitCode {
    #[brw(magic = 0x00u8)]
    Nds,
    #[brw(magic = 0x02u8)]
    NdsDsi,
    #[brw(magic = 0x03u8)]
    Dsi,
    Unknown(u8),
}
