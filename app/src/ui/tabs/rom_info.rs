use crate::ui::tabs::TabViewer;
use crate::ui::widgets::rom_actions::RomActions;
use crate::ui::widgets::rom_info::RomInfo;
use egui::Widget;

pub fn show(v: &mut TabViewer, ui: &mut egui::Ui) {
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(8, 8))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                RomActions::new(v.loaded_rom).ui(ui);
                RomInfo::new(v.loaded_rom).ui(ui);
            })
        });
}
