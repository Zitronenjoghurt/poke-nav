use egui::Widget;
use poke_nav::nds::fs::path::NdsPath;

pub struct NdsPathWidget<'a> {
    path: &'a NdsPath,
}

impl<'a> NdsPathWidget<'a> {
    pub fn new(path: &'a NdsPath) -> Self {
        Self { path }
    }
}

impl<'a> Widget for NdsPathWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let frame = egui::Frame::new()
            .fill(ui.visuals().extreme_bg_color)
            .inner_margin(egui::Margin::symmetric(8, 4))
            .corner_radius(4.0);
        let mut prepared = frame.begin(ui);
        prepared
            .content_ui
            .set_width(prepared.content_ui.available_width());
        prepared.content_ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            let segments: Vec<&str> = self
                .path
                .as_ref()
                .split('/')
                .filter(|s| !s.is_empty())
                .collect();
            for (i, segment) in segments.iter().enumerate() {
                if i > 0 {
                    ui.label(egui::RichText::new(" › ").weak());
                }
                let text = if i == segments.len() - 1 {
                    egui::RichText::new(*segment).monospace().strong()
                } else {
                    egui::RichText::new(*segment).monospace().weak()
                };
                ui.label(text);
            }
        });
        prepared.end(ui)
    }
}
