use egui::{Grid, Response, Ui, Widget};
use poke_nav::codec::common::fmt::format_bytes;
use poke_nav::codec::nds::formats::ParsedNdsFile;
use poke_nav::codec::nds::fs::file::{NdsFile, NdsFileData};

pub struct NdsFileInfo<'a> {
    file: &'a NdsFile,
}

impl<'a> NdsFileInfo<'a> {
    pub fn new(file: &'a NdsFile) -> Self {
        Self { file }
    }

    fn extra_info(&self, ui: &mut Ui) {
        let NdsFileData::Parsed { parsed, .. } = &self.file.data else {
            return;
        };
        match parsed {
            ParsedNdsFile::Narc(narc) => {
                ui.label("Directories");
                ui.label(narc.fs.directories.len().to_string());
                ui.end_row();

                ui.label("Files");
                ui.label(narc.fs.files.len().to_string());
                ui.end_row();

                ui.label("Version");
                ui.label(narc.header.version.to_string());
                ui.end_row();

                ui.label("Chunk size");
                ui.label(narc.header.chunk_size.to_string());
                ui.end_row();

                ui.label("Chunk count");
                ui.label(narc.header.num_chunks.to_string());
                ui.end_row();
            }
            ParsedNdsFile::HgSsMap(map) => {
                ui.label("Permission size");
                ui.label(map.header.permission_size.to_string());
                ui.end_row();

                ui.label("Objects size");
                ui.label(map.header.objects_size.to_string());
                ui.end_row();

                ui.label("Objects count");
                ui.label(map.objects.len().to_string());
                ui.end_row();

                ui.label("NSBMD size");
                ui.label(map.header.nsbmd_size.to_string());
                ui.end_row();

                ui.label("BDHC size");
                ui.label(map.header.bdhc_size.to_string());
                ui.end_row();

                ui.label("Unknown section size");
                ui.label(
                    map.unknown_data
                        .as_ref()
                        .map(|v| v.len().to_string())
                        .unwrap_or("0".to_string()),
                );
                ui.end_row();
            }
        }
    }

    fn visualization(&self, ui: &mut Ui) {
        let NdsFileData::Parsed { parsed, .. } = &self.file.data else {
            return;
        };
        match parsed {
            ParsedNdsFile::HgSsMap(map) => {
                egui::Frame::new()
                    .fill(ui.visuals().extreme_bg_color)
                    .inner_margin(4.0)
                    .corner_radius(4.0)
                    .show(ui, |ui| {
                        ui.monospace(
                            egui::RichText::new(map.permissions.to_grid_string())
                                .color(ui.visuals().strong_text_color()),
                        );
                    });
            }
            _ => {}
        }
    }
}

impl<'a> Widget for NdsFileInfo<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            Grid::new("nds_file_info_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label(self.file.name_with_ext_fallback());
                    ui.end_row();

                    ui.label("Size");
                    ui.label(format_bytes(self.file.size));
                    ui.end_row();

                    if let Some(format) = self.file.data.format() {
                        ui.label("Format Name");
                        ui.label(format.full_name());
                        ui.end_row();

                        ui.label("Format Description");
                        ui.label(format.explanation());
                        ui.end_row();
                    }

                    self.extra_info(ui);
                });

            ui.separator();

            self.visualization(ui);
        })
        .response
    }
}
