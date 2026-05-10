use crate::ui::icons;
use egui::{Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use poke_nav::nds::games::hgss::HgSsRom;

pub struct HgSsMapHeadersWidget<'a> {
    rom: &'a HgSsRom<'a>,
}

impl<'a> HgSsMapHeadersWidget<'a> {
    pub fn new(rom: &'a HgSsRom<'a>) -> Self {
        Self { rom }
    }
}

const HEADERS: [&str; 20] = [
    "#",
    "Wild",
    "Area",
    "MapX",
    "MapY",
    "Matrix",
    "Script",
    "LvlScript",
    "TextArc",
    "MusicDay",
    "MusicNight",
    "Event",
    "Name",
    "Icon",
    "Kanto",
    "Weather",
    "LocType",
    "Camera",
    "Follow",
    "BattleBG",
];

impl Widget for HgSsMapHeadersWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let headers = self.rom.read_map_headers();

        let available = ui.available_size();
        ui.scope(|ui| {
            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .min_scrolled_height(0.0)
                .max_scroll_height(available.y)
                .columns(Column::auto().clip(true), HEADERS.len())
                .header(20.0, |mut row| {
                    for label in HEADERS {
                        row.col(|ui| {
                            ui.strong(label);
                        });
                    }
                })
                .body(|body| {
                    body.rows(18.0, headers.len(), |mut row| {
                        let i = row.index();
                        let h = &headers[i];

                        row.col(|ui| {
                            ui.label(i.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.wild_pokemon_file_number.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.area_data_value.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.world_map_x().to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.world_map_y().to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.matrix_number.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.script_file_number.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.level_script_file.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.text_archive_number.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.day_music_track_number.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.night_music_track_number.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.event_file_number.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.map_name_index.to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.area_icon().to_string());
                        });
                        row.col(|ui| {
                            ui.label(if h.kanto_flag() {
                                icons::CHECK
                            } else {
                                icons::X
                            });
                        });
                        row.col(|ui| {
                            ui.label(h.weather_id().to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.location_type().to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.camera_angle_id().to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.follow_mode().to_string());
                        });
                        row.col(|ui| {
                            ui.label(h.battle_background().to_string());
                        });
                    });
                })
        })
        .response
    }
}
