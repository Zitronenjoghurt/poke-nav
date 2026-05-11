use crate::ui::widgets::text_grid_frame::TextGridFrame;
use egui::{CollapsingHeader, Grid, Response, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use poke_nav::fmt::format_bytes_long;
use poke_nav::platform::nds::formats::nstex::palette::NsPalette;
use poke_nav::platform::nds::formats::nstex::texture::NsTexture;
use poke_nav::platform::nds::formats::nstex::Nstex;
use poke_nav::platform::nds::formats::ParsedNdsFile;
use poke_nav::platform::nds::fs::file::{NdsFile, NdsFileData};

pub struct NdsFileInfo<'a> {
    file: &'a NdsFile,
}

impl<'a> NdsFileInfo<'a> {
    pub fn new(file: &'a NdsFile) -> Self {
        Self { file }
    }

    fn extra_info(&self, ui: &mut Ui) {
        let NdsFileData::Parsed { parsed, .. } = &self.file.data else {
            return;
        };
        match parsed {
            ParsedNdsFile::Narc(narc) => {
                ui.label("Directories");
                ui.label(narc.fs.directories.len().to_string());
                ui.end_row();

                ui.label("Files");
                ui.label(narc.fs.files.len().to_string());
                ui.end_row();

                ui.label("Version");
                ui.label(narc.header.version.to_string());
                ui.end_row();

                ui.label("Chunk size");
                ui.label(narc.header.chunk_size.to_string());
                ui.end_row();

                ui.label("Chunk count");
                ui.label(narc.header.num_chunks.to_string());
                ui.end_row();
            }
            ParsedNdsFile::Nsbtx(nsbtx) => {
                self.nstex_info(ui, &nsbtx.texture);
            }
            ParsedNdsFile::Nstex(nstex) => {
                self.nstex_info(ui, nstex);
            }
            ParsedNdsFile::Gen4MapData(map) => {
                ui.label("Permission size");
                ui.label(map.header.permission_size.to_string());
                ui.end_row();

                ui.label("Objects size");
                ui.label(map.header.objects_size.to_string());
                ui.end_row();

                ui.label("Objects count");
                ui.label(map.objects.len().to_string());
                ui.end_row();

                ui.label("NSBMD size");
                ui.label(map.header.nsbmd_size.to_string());
                ui.end_row();

                ui.label("BDHC size");
                ui.label(map.header.bdhc_size.to_string());
                ui.end_row();

                ui.label("Background sound section size");
                ui.label(
                    map.background_sound_section
                        .as_ref()
                        .map(|v| v.len().to_string())
                        .unwrap_or("0".to_string()),
                );
                ui.end_row();
            }
            ParsedNdsFile::Gen4MapMatrix(matrix) => {
                ui.label("Map prefix");
                ui.label(&matrix.header.prefix_name);
                ui.end_row();

                ui.label("Map width");
                ui.label(matrix.header.global_map_width.to_string());
                ui.end_row();

                ui.label("Map height");
                ui.label(matrix.header.global_map_height.to_string());
                ui.end_row();

                ui.label("Headers section flag");
                ui.label(matrix.header.headers_section_present.to_string());
                ui.end_row();

                ui.label("Altitudes section flag");
                ui.label(matrix.header.altitudes_section_present.to_string());
                ui.end_row();
            }
        }
    }

    fn visualization(&self, ui: &mut Ui) {
        let NdsFileData::Parsed { parsed, .. } = &self.file.data else {
            return;
        };
        match parsed {
            ParsedNdsFile::Gen4MapData(map) => {
                TextGridFrame::new(
                    "nds_file_info_gen_4_map_permissions",
                    &map.permissions.format_grid(),
                )
                .ui(ui);
            }
            ParsedNdsFile::Nsbtx(nsbtx) => {
                self.nstex_vis(ui, &nsbtx.texture);
            }
            ParsedNdsFile::Nstex(nstex) => {
                self.nstex_vis(ui, nstex);
            }
            ParsedNdsFile::Gen4MapMatrix(mat) => {
                CollapsingHeader::new("File IDs").show(ui, |ui| {
                    TextGridFrame::new("nds_file_info_gen_4_map_files", &mat.format_file_ids())
                        .ui(ui);
                });

                if let Some(headers) = mat.format_header_ids() {
                    CollapsingHeader::new("Header IDs").show(ui, |ui| {
                        TextGridFrame::new("nds_file_info_gen_4_map_headers", &headers).ui(ui);
                    });
                }
            }
            _ => {}
        }
    }

    fn nstex_info(&self, ui: &mut Ui, nstex: &Nstex) {
        ui.label("Texture chunk size");
        ui.label(nstex.header.chunk_size.to_string());
        ui.end_row();

        ui.label("Texture data size");
        ui.label(nstex.header.texture_data_size().to_string());
        ui.end_row();

        ui.label("Compressed texture data size");
        ui.label(nstex.header.compressed_texture_data_size().to_string());
        ui.end_row();

        ui.label("Palette data size");
        ui.label(nstex.header.palette_data_size().to_string());
        ui.end_row();
    }

    fn nstex_vis(&self, ui: &mut Ui, nstex: &Nstex) {
        CollapsingHeader::new("Texture Dictionary").show(ui, |ui| {
            let mut textures: Vec<&NsTexture> = nstex.textures.iter().collect();
            textures.sort_by_key(|t| &t.name);

            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::remainder())
                .min_scrolled_height(0.0)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Offset");
                    });
                    header.col(|ui| {
                        ui.strong("Width");
                    });
                    header.col(|ui| {
                        ui.strong("Height");
                    });
                    header.col(|ui| {
                        ui.strong("Format");
                    });
                    header.col(|ui| {
                        ui.strong("S-Size");
                    });
                    header.col(|ui| {
                        ui.strong("T-Size");
                    });
                    header.col(|ui| {
                        ui.strong("Transp. 0");
                    });
                })
                .body(|mut body| {
                    for tex in &textures {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&tex.name);
                            });
                            row.col(|ui| {
                                ui.label(format!("0x{:X}", tex.params.texture_data_offset()));
                            });
                            row.col(|ui| {
                                ui.label(tex.params.width().to_string());
                            });
                            row.col(|ui| {
                                ui.label(tex.params.height().to_string());
                            });
                            row.col(|ui| {
                                ui.label(tex.params.format().to_string());
                            });
                            row.col(|ui| {
                                ui.label(tex.params.s_size().to_string());
                            });
                            row.col(|ui| {
                                ui.label(tex.params.t_size().to_string());
                            });
                            row.col(|ui| {
                                ui.label(tex.params.is_color_0_transparent().to_string());
                            });
                        });
                    }
                });
        });

        CollapsingHeader::new("Palette Dictionary").show(ui, |ui| {
            let mut palettes: Vec<&NsPalette> = nstex.palettes.iter().collect();
            palettes.sort_by_key(|p| &p.name);

            TableBuilder::new(ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::remainder())
                .min_scrolled_height(0.0)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Offset");
                    });
                })
                .body(|mut body| {
                    for pal in &palettes {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label(&pal.name);
                            });
                            row.col(|ui| {
                                ui.label(format!("0x{:X}", pal.offset));
                            });
                        });
                    }
                });
        });
    }
}

impl<'a> Widget for NdsFileInfo<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            Grid::new("nds_file_info_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Name");
                    ui.label(self.file.name_with_ext_fallback());
                    ui.end_row();

                    ui.label("Size");
                    ui.label(format_bytes_long(self.file.size));
                    ui.end_row();

                    if let Some(format) = self.file.data.format() {
                        ui.label("Format Name");
                        ui.label(format.full_name());
                        ui.end_row();

                        ui.label("Format Description");
                        ui.label(format.explanation());
                        ui.end_row();
                    }

                    self.extra_info(ui);
                });

            ui.separator();

            self.visualization(ui);
        })
        .response
    }
}
