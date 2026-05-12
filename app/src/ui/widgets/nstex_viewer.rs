use crate::ui::widgets::nstex_mode_selector::NstexModeSelector;
use crate::utils::textures::TextureCache;
use egui::{Response, Ui};
use poke_nav::fmt::format_bytes;
use poke_nav::platform::nds::formats::nstex::{Nstex, NstexDecodeMode};
use poke_nav::platform::nds::fs::path::NdsPath;

pub struct NstexViewer<'a> {
    path: &'a NdsPath,
    nstex: &'a Nstex,
    mode: &'a mut NstexDecodeMode,
    cache: &'a mut TextureCache,
}

impl<'a> NstexViewer<'a> {
    pub fn new(
        path: &'a NdsPath,
        nstex: &'a Nstex,
        mode: &'a mut NstexDecodeMode,
        cache: &'a mut TextureCache,
    ) -> Self {
        Self {
            path,
            nstex,
            mode,
            cache,
        }
    }
}

impl egui::Widget for NstexViewer<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            NstexModeSelector::new(self.path, self.nstex, self.mode).ui(ui);

            ui.separator();

            match self.cache.nstex(ui.ctx(), self.path, self.nstex, self.mode) {
                Ok(handle) => {
                    let size = handle.size_vec2();
                    let available = ui.available_size();
                    let max_scale = (available.x / size.x)
                        .min(available.y / size.y)
                        .floor()
                        .max(1.0);
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.image(egui::load::SizedTexture::new(handle.id(), size * max_scale));
                        ui.monospace(format!(
                            "{}x{} {}",
                            size.x,
                            size.y,
                            format_bytes(handle.byte_size())
                        ));
                    });
                }
                Err(e) => {
                    ui.colored_label(egui::Color32::RED, format!("Error: {e}"));
                }
            }
        })
        .response
    }
}
