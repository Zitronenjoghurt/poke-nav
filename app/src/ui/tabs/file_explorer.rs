use crate::ui::tabs::TabViewer;
use crate::ui::widgets::nds_file_explorer::NdsFileExplorer;
use crate::utils::file_picker::FilePicker;
use crate::utils::task::TaskUi;
use egui::Widget;

pub fn show(v: &mut TabViewer, ui: &mut egui::Ui) {
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(8, 8))
        .show(ui, |ui| match v.loaded_rom.show(ui) {
            TaskUi::Idle => {
                if ui.button("Load ROM").clicked() {
                    FilePicker::pick_rom(v.loaded_rom, ui);
                }
            }
            TaskUi::Handled(_) => {}
            TaskUi::Done(rom) => {
                if let Some(nds) = rom.nds() {
                    NdsFileExplorer::new(v.toasts, &nds.fs, &mut v.state).ui(ui);
                } else {
                    ui.label("ROM has no file system.");
                }
            }
        });
}
