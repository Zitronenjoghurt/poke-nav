use crate::ui::tabs::TabViewer;
use crate::ui::widgets::nstex_viewer::NstexViewer;
use egui::Widget;

pub fn show(v: &mut TabViewer, ui: &mut egui::Ui) {
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(8, 8))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                if let Some(path) = &v.state.selected_file_explorer_path
                    && let Some(rom) = v.loaded_rom.get()
                    && let Some(nds) = rom.nds()
                    && let Some(file) = nds.fs.get_file(path.clone())
                    && let Some(nstex) = file.data.nstex()
                {
                    NstexViewer::new(nstex, &mut v.state.selected_nstex_ref, v.texture_cache)
                        .ui(ui);
                } else {
                    ui.label("No texture selected.");
                }
            })
        });
}
