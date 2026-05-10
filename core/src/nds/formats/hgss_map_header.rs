use binrw::binrw;

#[binrw]
#[brw(little)]
pub struct HGSSMapHeader {
    /// 0x00: Index into the wild Pokémon encounter file table (0xFF = no encounters)
    pub wild_pokemon_file_number: u8,
    /// 0x01: Area data ID, controls area-specific properties (tileset, etc.)
    pub area_data_value: u8,
    /// 0x02-0x03: Packed world map coordinates
    ///   bits 0-3:   unknown0 (4 bits)
    ///   bits 4-9:   worldmapX (6 bits)
    ///   bits 10-15: worldmapY (6 bits)
    pub coords: u16,
    /// 0x04-0x05: Index of the map matrix (defines map tile layout)
    pub matrix_number: u16,
    /// 0x06-0x07: Index of the map's script file
    pub script_file_number: u16,
    /// 0x08-0x09: Index of the map's level script file (background/trigger scripts)
    pub level_script_file: u16,
    /// 0x0A-0x0B: Index of the text archive used by this map's scripts
    pub text_archive_number: u16,
    /// 0x0C-0x0D: Music track ID played during daytime
    pub day_music_track_number: u16,
    /// 0x0E-0x0F: Music track ID played during nighttime
    pub night_music_track_number: u16,
    /// 0x10-0x11: Index of the event file (NPCs, warps, triggers)
    pub event_file_number: u16,
    /// 0x12: Index into Text Archive #382 for the map's display name
    pub map_name_index: u8,
    /// 0x13: Packed area properties
    ///   bits 0-3: area icon displayed on the map name popup (4 bits)
    ///   bits 4-7: unknown1 (4 bits)
    pub area_properties: u8,
    /// 0x14-0x17: Packed flags and properties
    ///   bit 0:      Kanto flag (1 = Kanto region map)
    ///   bits 1-7:   weather ID (7 bits)
    ///   bits 8-11:  location type (4 bits)
    ///   bits 12-17: camera angle ID (6 bits)
    ///   bits 18-19: follow mode for partner Pokémon (2 bits)
    ///   bits 20-24: battle background ID (5 bits)
    ///   bits 25-31: permission flags (7 bits) — fly, esc rope, bicycle, etc.
    pub packed_flags: u32,
}

impl HGSSMapHeader {
    pub fn kanto_flag(&self) -> bool {
        self.packed_flags & 1 == 1
    }

    pub fn weather_id(&self) -> u8 {
        ((self.packed_flags >> 1) & 0x7F) as u8
    }

    pub fn location_type(&self) -> u8 {
        ((self.packed_flags >> 8) & 0xF) as u8
    }

    pub fn camera_angle_id(&self) -> u8 {
        ((self.packed_flags >> 12) & 0x3F) as u8
    }

    pub fn follow_mode(&self) -> u8 {
        ((self.packed_flags >> 18) & 0x3) as u8
    }

    pub fn battle_background(&self) -> u8 {
        ((self.packed_flags >> 20) & 0x1F) as u8
    }

    pub fn flags(&self) -> u8 {
        ((self.packed_flags >> 25) & 0x7F) as u8
    }

    pub fn world_map_x(&self) -> u8 {
        ((self.coords >> 4) & 0x3F) as u8
    }

    pub fn world_map_y(&self) -> u8 {
        ((self.coords >> 10) & 0x3F) as u8
    }

    pub fn area_icon(&self) -> u8 {
        self.area_properties & 0xF
    }
}
