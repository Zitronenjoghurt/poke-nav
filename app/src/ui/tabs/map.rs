use crate::ui::tabs::TabViewer;
use crate::ui::widgets::rom_info::RomInfo;
use egui::Widget;

pub fn show(viewer: &mut TabViewer, ui: &mut egui::Ui) {
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(8, 8))
        .show(ui, |ui| RomInfo::new(viewer.loaded_rom).ui(ui));
}
