use binrw::binrw;

#[binrw]
#[brw(little)]
pub struct HGSSMapHeader {
    pub wild_pokemon_file_number: u8,
    pub area_data_value: u8,
    pub unknown: [u8; 2],
    pub matrix_number: u16,
    pub script_file_number: u16,
    pub level_script_file: u16,
    pub text_archive_number: u16,
    pub day_music_track_number: u16,
    pub night_music_track_number: u16,
    pub event_file_number: u16,
    pub map_name_index: u8,
    pub map_name_textbox_type: u8,
    pub weather_value: u8,
    pub camera_value: u8,
    pub follow_mode: u8,
    pub permission_flags: u8,
}
