use crate::ui::icons;
use egui::{Id, Response, Ui};
use egui_ltreeview::{NodeBuilder, TreeView};
use poke_nav::codec::nds::formats::{NdsFileFormat, ParsedNdsFile};
use poke_nav::codec::nds::fs::file::NdsFileData;
use poke_nav::codec::nds::fs::{NdsFileSystem, ROOT_DIR_ID};

pub struct NdsFsTree<'a> {
    id: &'a str,
    fs: &'a NdsFileSystem,
}

impl<'a> NdsFsTree<'a> {
    pub fn new(id: &'a str, fs: &'a NdsFileSystem) -> Self {
        Self { id, fs }
    }

    pub fn show(self, ui: &mut Ui) -> NdsFsTreeResponse {
        let (response, actions) = TreeView::new(Id::new(self.id)).show(ui, |builder| {
            let open = builder.node(
                NodeBuilder::dir(String::new())
                    .default_open(true)
                    .icon(|ui| {
                        ui.label(icons::FOLDER);
                    })
                    .label(""),
            );
            if open {
                build_dir(builder, self.fs, ROOT_DIR_ID, String::new());
            }
            builder.close_dir();
        });

        let mut selected_path = None;
        for action in actions {
            if let egui_ltreeview::Action::SetSelected(paths) = action
                && let Some(path) = paths.first()
            {
                selected_path = Some(path.to_string());
            }
        }

        NdsFsTreeResponse {
            response,
            selected_path,
        }
    }
}

pub struct NdsFsTreeResponse {
    pub response: Response,
    pub selected_path: Option<String>,
}

fn build_dir(
    builder: &mut egui_ltreeview::TreeViewBuilder<String>,
    fs: &NdsFileSystem,
    dir_id: u16,
    prefix: String,
) {
    let mut child_dirs: Vec<_> = fs
        .directories
        .iter()
        .filter(|d| d.parent_dir_id == dir_id && d.id != dir_id)
        .collect();
    child_dirs.sort_by(|a, b| a.name.cmp(&b.name));

    let mut child_files: Vec<_> = fs
        .files
        .iter()
        .filter(|f| f.parent_dir_id == dir_id)
        .collect();
    child_files.sort_by(|a, b| a.name.cmp(&b.name));

    for dir in child_dirs {
        let path = format!("{}/{}", prefix, dir.name);
        let open = builder.node(
            NodeBuilder::dir(path.clone())
                .default_open(false)
                .icon(|ui| {
                    ui.label(icons::FOLDER);
                })
                .label(&dir.name),
        );
        if open {
            build_dir(builder, fs, dir.id, path);
        }
        builder.close_dir();
    }

    for file in child_files {
        let path = format!("{}/{}", prefix, file.name);

        match &file.data {
            NdsFileData::Parsed {
                parsed: ParsedNdsFile::Narc(narc),
                ..
            } => {
                let open = builder.node(
                    NodeBuilder::dir(path.clone())
                        .default_open(false)
                        .icon(|ui| {
                            ui.label(icons::FILE_ARCHIVE);
                        })
                        .label(&file.name),
                );
                if open {
                    build_dir(builder, &narc.fs, ROOT_DIR_ID, path);
                }
                builder.close_dir();
            }

            NdsFileData::Parsed { parsed, .. } => {
                let icon = format_icon(parsed);
                builder.node(
                    NodeBuilder::leaf(path)
                        .icon(move |ui| {
                            ui.label(icon);
                        })
                        .label(&file.name),
                );
            }

            NdsFileData::Raw(_) => {
                builder.node(
                    NodeBuilder::leaf(path)
                        .icon(|ui| {
                            ui.label(egui_phosphor::regular::FILE);
                        })
                        .label(&file.name),
                );
            }
        }
    }
}

fn format_icon(parsed: &ParsedNdsFile) -> &'static str {
    match parsed.format() {
        NdsFileFormat::Narc => icons::FILE_ARCHIVE,
        NdsFileFormat::HgSsMap => icons::MAP_TRIFOLD,
    }
}
