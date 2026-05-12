use crate::utils::textures::TextureCache;
use egui::{Grid, Response, Ui, Widget};
use poke_nav::fmt::format_bytes_long;

pub struct DebugWidget<'a> {
    pub texture_cache: &'a mut TextureCache,
}

impl Widget for DebugWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        Grid::new("debug_grid")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Texture Cache Size");
                ui.label(format_bytes_long(self.texture_cache.size()));
            })
            .response
    }
}
