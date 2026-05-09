use crate::ui::widgets::nds_file_actions::NdsFileActions;
use crate::ui::widgets::nds_fs_tree::NdsFsTree;
use egui::{Response, Ui};
use poke_nav::codec::nds::fs::path::NdsPath;
use poke_nav::codec::nds::fs::{NdsFileSystem, NdsFileSystemEntry};

pub struct NdsFileExplorer<'a> {
    fs: &'a NdsFileSystem,
    selected_path: &'a mut Option<NdsPath>,
}

impl<'a> NdsFileExplorer<'a> {
    pub fn new(fs: &'a NdsFileSystem, selected_path: &'a mut Option<NdsPath>) -> Self {
        Self { fs, selected_path }
    }
}

impl<'a> egui::Widget for NdsFileExplorer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            if let Some(path) = &self.selected_path
                && let Some(entry) = self.fs.get_entry(path.clone())
            {
                match entry {
                    NdsFileSystemEntry::File(file) => {
                        NdsFileActions::new(file).ui(ui);
                    }
                    NdsFileSystemEntry::Directory(dir) => {}
                }
            }

            ui.separator();

            let tree_response = NdsFsTree::new("nds_fs_tree", self.fs).show(ui);
            if let Some(path) = tree_response.selected_path {
                *self.selected_path = Some(path.into());
            };
        })
        .response
    }
}
