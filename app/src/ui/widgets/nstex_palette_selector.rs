use egui::Widget;
use poke_nav::platform::nds::formats::nstex::Nstex;

pub struct NstexPaletteSelector<'a> {
    nstex: &'a Nstex,
    palette_index: &'a mut Option<usize>,
    enabled: bool,
}

impl<'a> NstexPaletteSelector<'a> {
    pub fn new(nstex: &'a Nstex, palette_index: &'a mut Option<usize>) -> Self {
        Self {
            nstex,
            palette_index,
            enabled: true,
        }
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for NstexPaletteSelector<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.add_enabled_ui(self.enabled, |ui| {
            let pal_label = match self.palette_index {
                Some(i) => self
                    .nstex
                    .palettes
                    .get(*i)
                    .map(|p| format!("[{}] {}", i, &p.name))
                    .unwrap_or_else(|| "???".into()),
                None => "None".into(),
            };

            ui.vertical(|ui| {
                ui.label("Palette");
                egui::ComboBox::from_id_salt("nstex_palette")
                    .selected_text(&pal_label)
                    .show_ui(ui, |ui| {
                        for (i, pal) in self.nstex.palettes.iter().enumerate() {
                            let selected = *self.palette_index == Some(i);
                            if ui
                                .selectable_label(selected, format!("[{}] {}", i, &pal.name))
                                .clicked()
                            {
                                *self.palette_index = Some(i);
                            }
                        }
                    });
            });
        })
        .response
    }
}
