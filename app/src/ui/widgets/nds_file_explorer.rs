use crate::ui::state::UiState;
use crate::ui::widgets::nds_dir_actions::NdsDirActions;
use crate::ui::widgets::nds_file_actions::NdsFileActions;
use crate::ui::widgets::nds_fs_tree::NdsFsTree;
use crate::ui::widgets::nds_fs_tree_filter::NdsFsTreeFilter;
use crate::ui::widgets::nds_path::NdsPathWidget;
use egui::{Response, ScrollArea, Ui};
use poke_nav::platform::nds::fs::{NdsFileSystem, NdsFileSystemEntry};

pub struct NdsFileExplorer<'a> {
    toasts: &'a mut egui_notify::Toasts,
    fs: &'a NdsFileSystem,
    state: &'a mut UiState,
}

impl<'a> NdsFileExplorer<'a> {
    pub fn new(
        toasts: &'a mut egui_notify::Toasts,
        fs: &'a NdsFileSystem,
        state: &'a mut UiState,
    ) -> Self {
        Self { toasts, fs, state }
    }
}

impl<'a> egui::Widget for NdsFileExplorer<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            NdsFsTreeFilter::new(&mut self.state.nds_fs_tree.filter).ui(ui);
            ui.separator();

            if let Some(path) = &self.state.selected_file_explorer_path
                && let Some(entry) = self.fs.get_entry(path.clone())
            {
                ui.vertical(|ui| {
                    NdsPathWidget::new(path).ui(ui);
                    match entry {
                        NdsFileSystemEntry::File(file) => {
                            NdsFileActions::new(file, self.toasts).ui(ui);
                        }
                        NdsFileSystemEntry::Directory(dir) => {
                            NdsDirActions::new(self.fs, dir, self.toasts).ui(ui);
                        }
                    }
                });
                ui.separator();
            }

            ScrollArea::vertical()
                .min_scrolled_width(ui.available_width())
                .show(ui, |ui| {
                    let tree_response =
                        NdsFsTree::new("nds_fs_tree", self.fs, &mut self.state.nds_fs_tree)
                            .show(ui);
                    if let Some(path) = tree_response.selected_path {
                        self.state.selected_file_explorer_path = Some(path.into());
                    };
                });
        })
        .response
    }
}
