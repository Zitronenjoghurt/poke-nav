use crate::ui::widgets::nstex_ref_selector::NstexRefSelector;
use crate::utils::textures::TextureCache;
use egui::{Response, Ui};
use poke_nav::platform::nds::formats::nstex::{Nstex, NstexRef};

pub struct NstexViewer<'a> {
    nstex: &'a Nstex,
    selected_ref: &'a mut NstexRef,
    cache: &'a mut TextureCache,
}

impl<'a> NstexViewer<'a> {
    pub fn new(
        nstex: &'a Nstex,
        selected_ref: &'a mut NstexRef,
        cache: &'a mut TextureCache,
    ) -> Self {
        Self {
            nstex,
            selected_ref,
            cache,
        }
    }
}

impl egui::Widget for NstexViewer<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            NstexRefSelector::new(self.nstex, self.selected_ref).ui(ui);

            match self.cache.nstex(ui.ctx(), self.nstex, self.selected_ref) {
                Ok(handle) => {
                    let size = handle.size_vec2();
                    let available = ui.available_size();
                    let max_scale = (available.x / size.x)
                        .min(available.y / size.y)
                        .floor()
                        .max(1.0);
                    ui.image(egui::load::SizedTexture::new(handle.id(), size * max_scale));
                }
                Err(e) => {
                    ui.colored_label(egui::Color32::RED, format!("Error: {e}"));
                }
            }
        })
        .response
    }
}
