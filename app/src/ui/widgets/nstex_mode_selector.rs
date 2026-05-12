use crate::ui::icons;
use crate::ui::widgets::nstex_palette_selector::NstexPaletteSelector;
use crate::utils::file_saver::FileSaver;
use egui::{Response, Widget};
use poke_nav::platform::nds::formats::nstex::{Nstex, NstexDecodeMode};
use poke_nav::platform::nds::fs::path::NdsPath;

pub struct NstexModeSelector<'a> {
    path: &'a NdsPath,
    nstex: &'a Nstex,
    mode: &'a mut NstexDecodeMode,
}

impl<'a> NstexModeSelector<'a> {
    pub fn new(path: &'a NdsPath, nstex: &'a Nstex, mode: &'a mut NstexDecodeMode) -> Self {
        Self { path, nstex, mode }
    }
}

impl Widget for NstexModeSelector<'_> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let response = ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(
                        matches!(self.mode, NstexDecodeMode::Single { .. }),
                        "Single",
                    )
                    .clicked()
                {
                    let palette_index = self.mode.palette_index();
                    *self.mode = NstexDecodeMode::Single {
                        texture_index: 0,
                        palette_index,
                    };
                }
                if ui
                    .selectable_label(matches!(self.mode, NstexDecodeMode::Sheet { .. }), "Sheet")
                    .clicked()
                {
                    let palette_index = self.mode.palette_index();
                    *self.mode = NstexDecodeMode::Sheet {
                        palette_index,
                        columns: None,
                    };
                }
            });

            ui.separator();

            ui.horizontal(|ui| match self.mode {
                NstexDecodeMode::Single {
                    texture_index,
                    palette_index,
                } => {
                    let tex_label = self
                        .nstex
                        .textures
                        .get(*texture_index)
                        .map(|t| format!("[{}] {}", texture_index, &t.name))
                        .unwrap_or_else(|| "???".into());

                    ui.vertical(|ui| {
                        ui.label("Texture");
                        egui::ComboBox::from_id_salt("nstex_texture")
                            .selected_text(&tex_label)
                            .show_ui(ui, |ui| {
                                for (i, tex) in self.nstex.textures.iter().enumerate() {
                                    ui.selectable_value(
                                        texture_index,
                                        i,
                                        format!("[{}] {}", i, &tex.name),
                                    );
                                }
                            });
                    });

                    let needs_palette = self
                        .nstex
                        .textures
                        .get(*texture_index)
                        .and_then(|t| t.format().ok())
                        .is_some_and(|f| f.requires_palette());

                    NstexPaletteSelector::new(self.nstex, palette_index)
                        .enabled(needs_palette)
                        .ui(ui);

                    if needs_palette && palette_index.is_none() && !self.nstex.palettes.is_empty() {
                        *palette_index = Some(0);
                    } else if !needs_palette {
                        *palette_index = None;
                    }

                    if ui.button("Export as PNG").clicked() {
                        let file_name = self.path.last_component().unwrap_or_default();
                        let texture_name =
                            if let Ok(texture) = self.nstex.get_texture(*texture_index) {
                                format!("-{}", texture.name)
                            } else {
                                "".to_string()
                            };
                        let palette_name = if let Some(Ok(palette)) =
                            palette_index.map(|i| self.nstex.get_palette(i))
                        {
                            format!("-{}", palette.name)
                        } else {
                            "".to_string()
                        };
                        let name = format!("{file_name}{texture_name}{palette_name}.png");
                        if let Ok(data) = self.nstex.decode(self.mode)
                            && let Ok(png) = data.render_png()
                        {
                            FileSaver::new()
                                .title("Save single texture")
                                .file_name(&name)
                                .dispatch(png);
                        }
                    }
                }
                NstexDecodeMode::Sheet {
                    palette_index,
                    columns,
                } => {
                    let any_needs_palette = self
                        .nstex
                        .textures
                        .iter()
                        .any(|t| t.format().ok().is_some_and(|f| f.requires_palette()));

                    NstexPaletteSelector::new(self.nstex, palette_index)
                        .enabled(any_needs_palette)
                        .ui(ui);

                    if any_needs_palette
                        && palette_index.is_none()
                        && !self.nstex.palettes.is_empty()
                    {
                        *palette_index = Some(0);
                    } else if !any_needs_palette {
                        *palette_index = None;
                    }

                    ui.vertical(|ui| {
                        ui.label("Columns");
                        let mut auto = columns.is_none();
                        let mut col_val = columns.unwrap_or_else(|| {
                            (self.nstex.textures.len() as f64).sqrt().ceil() as usize
                        });
                        ui.horizontal(|ui| {
                            ui.add_enabled(
                                !auto,
                                egui::DragValue::new(&mut col_val)
                                    .range(1..=self.nstex.textures.len().max(1)),
                            );
                            ui.toggle_value(&mut auto, icons::GRID_FOUR);
                            *columns = if auto { None } else { Some(col_val) };
                        });
                    });

                    if ui.button("Export as PNG").clicked() {
                        let file_name = self.path.last_component().unwrap_or_default();
                        let palette_name = if let Some(Ok(palette)) =
                            palette_index.map(|i| self.nstex.get_palette(i))
                        {
                            format!("-{}", palette.name)
                        } else {
                            "".to_string()
                        };
                        let name = format!("{file_name}{palette_name}.png");
                        if let Ok(data) = self.nstex.decode(self.mode)
                            && let Ok(png) = data.render_png()
                        {
                            FileSaver::new()
                                .title("Save sheet texture")
                                .file_name(&name)
                                .dispatch(png);
                        }
                    }
                }
            });
        });

        response.response
    }
}
