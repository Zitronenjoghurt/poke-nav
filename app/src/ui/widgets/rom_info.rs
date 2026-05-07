use crate::utils::file_picker::FilePicker;
use crate::utils::task::{Task, TaskUi};
use egui::{Grid, Response, Ui, Widget};
use poke_nav::codec::common::rom::Rom;

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
                    })
                    .response
            }
            TaskUi::Handled(r) => r,
        }
    }
}
