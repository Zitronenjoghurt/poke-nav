use egui::{Response, ScrollArea, Ui, Widget};

pub struct TextGridFrame<'a> {
    grid_text: &'a str,
}

impl<'a> TextGridFrame<'a> {
    pub fn new(grid_text: &'a str) -> Self {
        Self { grid_text }
    }
}

impl<'a> Widget for TextGridFrame<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ScrollArea::both()
            .min_scrolled_width(ui.available_width())
            .min_scrolled_height(ui.available_height())
            .show(ui, |ui| {
                egui::Frame::new()
                    .fill(ui.visuals().extreme_bg_color)
                    .inner_margin(4.0)
                    .corner_radius(4.0)
                    .show(ui, |ui| {
                        ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                        ui.monospace(
                            egui::RichText::new(self.grid_text)
                                .color(ui.visuals().strong_text_color()),
                        );
                    })
                    .response
            })
            .inner
    }
}
