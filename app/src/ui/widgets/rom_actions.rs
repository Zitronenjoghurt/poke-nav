use crate::utils::task::Task;
use egui::{Response, Ui};
use poke_nav::rom::Rom;

pub struct RomActions<'a> {
    rom: &'a mut Task<Rom>,
}

impl<'a> RomActions<'a> {
    pub fn new(rom: &'a mut Task<Rom>) -> Self {
        Self { rom }
    }
}

impl egui::Widget for RomActions<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let Some(rom) = self.rom.get() else {
                return;
            };

            if let Some(nds) = rom.nds()
                && let Some(hgss) = nds.hgss_rom()
            {
                if ui.button("Probe Map Header").clicked() {
                    if let Some(offset) = hgss.find_map_header_table_offset() {
                        println!("Map header table offset: 0x{:08X}", offset);
                    } else {
                        println!("Map header table not found");
                    }
                }
            }
        })
        .response
    }
}
