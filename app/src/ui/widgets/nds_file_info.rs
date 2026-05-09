use egui::{Grid, Response, Ui, Widget};
use poke_nav::codec::common::fmt::format_bytes;
use poke_nav::codec::nds::fs::file::NdsFile;

pub struct NdsFileInfo<'a> {
    file: &'a NdsFile,
}

impl<'a> NdsFileInfo<'a> {
    pub fn new(file: &'a NdsFile) -> Self {
        Self { file }
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
                });
        })
        .response
    }
}
