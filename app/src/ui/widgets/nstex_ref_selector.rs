use egui::{Response, Widget};
use poke_nav::platform::nds::formats::nstex::{Nstex, NstexRef};

pub struct NstexRefSelector<'a> {
    pub nstex: &'a Nstex,
    pub nstex_ref: &'a mut NstexRef,
}

impl<'a> NstexRefSelector<'a> {
    pub fn new(nstex: &'a Nstex, nstex_ref: &'a mut NstexRef) -> Self {
        Self { nstex, nstex_ref }
    }
}

impl Widget for NstexRefSelector<'_> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let Self { nstex, nstex_ref } = self;

        let response = ui.horizontal(|ui| {
            let tex_label = nstex
                .textures
                .get(nstex_ref.texture_index)
                .map(|t| format!("[{}] {}", nstex_ref.texture_index, &t.name))
                .unwrap_or_else(|| "???".into());

            ui.vertical(|ui| {
                ui.label("Texture");
                egui::ComboBox::from_id_salt("nstex_texture")
                    .selected_text(&tex_label)
                    .show_ui(ui, |ui| {
                        for (i, tex) in nstex.textures.iter().enumerate() {
                            ui.selectable_value(
                                &mut nstex_ref.texture_index,
                                i,
                                format!("[{}] {}", i, &tex.name),
                            );
                        }
                    });
            });

            let needs_palette = nstex
                .textures
                .get(nstex_ref.texture_index)
                .and_then(|t| t.format().ok())
                .is_some_and(|f| f.requires_palette());

            ui.add_enabled_ui(needs_palette, |ui| {
                let pal_label = match nstex_ref.palette_index {
                    Some(i) => nstex
                        .palettes
                        .get(i)
                        .map(|p| format!("[{}] {}", i, &p.name))
                        .unwrap_or_else(|| "???".into()),
                    None => "None".into(),
                };

                ui.vertical(|ui| {
                    ui.label("Palette");
                    egui::ComboBox::from_id_salt("nstex_palette")
                        .selected_text(&pal_label)
                        .show_ui(ui, |ui| {
                            for (i, pal) in nstex.palettes.iter().enumerate() {
                                let selected = nstex_ref.palette_index == Some(i);
                                if ui
                                    .selectable_label(selected, format!("[{}] {}", i, &pal.name))
                                    .clicked()
                                {
                                    nstex_ref.palette_index = Some(i);
                                }
                            }
                        });
                });
            });

            if needs_palette && nstex_ref.palette_index.is_none() && !nstex.palettes.is_empty() {
                nstex_ref.palette_index = Some(0);
            } else if !needs_palette {
                nstex_ref.palette_index = None;
            }
        });

        response.response
    }
}
