use crate::ui::tabs::TabViewer;
use crate::ui::widgets::debug_widget::DebugWidget;
use egui::Widget;

pub fn show(v: &mut TabViewer, ui: &mut egui::Ui) {
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(8, 8))
        .show(ui, |ui| {
            DebugWidget {
                texture_cache: v.texture_cache,
            }
            .ui(ui);
        });
}
