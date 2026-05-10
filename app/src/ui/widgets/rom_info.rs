use crate::utils::file_picker::FilePicker;
use crate::utils::task::{Task, TaskUi};
use egui::{Grid, Response, Ui, Widget};
use poke_nav::fmt::format_bytes_long;
use poke_nav::rom::Rom;

pub struct RomInfo<'a> {
    rom: &'a mut Task<Rom>,
}

impl<'a> RomInfo<'a> {
    pub fn new(rom: &'a mut Task<Rom>) -> Self {
        Self { rom }
    }
}

impl Widget for RomInfo<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        match self.rom.show(ui) {
            TaskUi::Idle => {
                let r = ui.button("Load ROM");
                if r.clicked() {
                    FilePicker::pick_rom(self.rom, ui);
                }
                r
            }
            TaskUi::Done(rom) => {
                Grid::new("rom_info_grid")
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label("Platform");
                        ui.label(rom.platform().to_string());
                        ui.end_row();

                        ui.label("Name");
                        ui.label(rom.name());
                        ui.end_row();

                        if let Some(nds) = rom.nds() {
                            if let Some(compressed_arm9_size) = nds.compressed_arm9_size {
                                ui.label("Compressed arm9 binary size");
                                ui.label(format_bytes_long(compressed_arm9_size));
                                ui.end_row();

                                ui.label("Decompressed arm9 binary size");
                                ui.label(format_bytes_long(nds.arm9_binary.len()));
                                ui.end_row();

                                ui.label("arm9 compression ratio");
                                ui.label(format!(
                                    "{:.2}%",
                                    compressed_arm9_size as f32 / nds.arm9_binary.len() as f32
                                        * 100.0
                                ));
                                ui.end_row();
                            } else {
                                ui.label("arm9 binary size");
                                ui.label(format_bytes_long(nds.arm9_binary.len()));
                                ui.end_row();
                            }

                            ui.label("arm7 binary size");
                            ui.label(format_bytes_long(nds.arm7_binary.len()));
                            ui.end_row();
                        }
                    })
                    .response
            }
            TaskUi::Handled(r) => r,
        }
    }
}
