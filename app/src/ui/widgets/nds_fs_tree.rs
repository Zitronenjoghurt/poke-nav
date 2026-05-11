use crate::ui::icons;
use egui::{Id, Response, Ui};
use egui_ltreeview::{NodeBuilder, TreeView};
use poke_nav::platform::nds::formats::NdsFileFormat;
use poke_nav::platform::nds::fs::tree::{NdsFileTree, NdsFileTreeEntry, NdsFileTreeFilter};
use poke_nav::platform::nds::fs::NdsFileSystem;

pub struct NdsFsTree<'a> {
    id: &'a str,
    fs: &'a NdsFileSystem,
    state: &'a mut NdsFsTreeState,
}

impl<'a> NdsFsTree<'a> {
    pub fn new(id: &'a str, fs: &'a NdsFileSystem, state: &'a mut NdsFsTreeState) -> Self {
        Self { id, fs, state }
    }

    pub fn show(self, ui: &mut Ui) -> NdsFsTreeResponse {
        self.state.rebuild(self.fs, ui);

        let (response, actions) = TreeView::new(Id::new(self.id)).show(ui, |builder| {
            let open = builder.node(
                NodeBuilder::dir(self.state.tree.root.path.clone())
                    .default_open(true)
                    .icon(|ui| {
                        ui.label(icons::FOLDER);
                    })
                    .label(""),
            );
            if open {
                build_entries(builder, &self.state.tree.root.children);
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

fn build_entries(
    builder: &mut egui_ltreeview::TreeViewBuilder<String>,
    entries: &[NdsFileTreeEntry],
) {
    for entry in entries {
        match entry {
            NdsFileTreeEntry::Dir(dir) => {
                let open = builder.node(
                    NodeBuilder::dir(dir.path.clone())
                        .default_open(false)
                        .icon(|ui| {
                            ui.label(icons::FOLDER);
                        })
                        .label(&dir.name),
                );
                if open {
                    build_entries(builder, &dir.children);
                }
                builder.close_dir();
            }
            NdsFileTreeEntry::File(file) => {
                let icon = file
                    .format
                    .as_ref()
                    .map(format_icon)
                    .unwrap_or(egui_phosphor::regular::FILE);

                if let Some(children) = &file.children {
                    let open = builder.node(
                        NodeBuilder::dir(file.path.clone())
                            .default_open(false)
                            .icon(move |ui| {
                                ui.label(icon);
                            })
                            .label(&file.name),
                    );
                    if open {
                        build_entries(builder, children);
                    }
                    builder.close_dir();
                } else {
                    builder.node(
                        NodeBuilder::leaf(file.path.clone())
                            .icon(move |ui| {
                                ui.label(icon);
                            })
                            .label(&file.name),
                    );
                }
            }
        }
    }
}

fn format_icon(format: &NdsFileFormat) -> &'static str {
    match format {
        NdsFileFormat::Narc => icons::FILE_ARCHIVE,
        NdsFileFormat::Gen4MapData => icons::MAP_TRIFOLD,
        NdsFileFormat::Gen4MapMatrix => icons::SQUARES_FOUR,
    }
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct NdsFsTreeState {
    #[serde(skip, default)]
    pub tree: NdsFileTree,
    pub filter: NdsFileTreeFilter,
}

impl NdsFsTreeState {
    pub fn rebuild(&mut self, fs: &NdsFileSystem, ui: &mut Ui) {
        if !self.filter.dirty {
            return;
        }
        self.tree = NdsFileTree::build(fs, &self.filter);
        self.filter.dirty = false;
    }
}
