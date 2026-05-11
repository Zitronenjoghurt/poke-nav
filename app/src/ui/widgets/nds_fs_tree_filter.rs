use crate::ui::widgets::generic_multi_select::GenericMultiSelect;
use egui::Widget;
use poke_nav::platform::nds::fs::tree::NdsFileTreeFilter;

pub struct NdsFsTreeFilter<'a> {
    filter: &'a mut NdsFileTreeFilter,
}

impl<'a> NdsFsTreeFilter<'a> {
    pub fn new(filter: &'a mut NdsFileTreeFilter) -> Self {
        Self { filter }
    }
}

impl Widget for NdsFsTreeFilter<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                self.filter.dirty |= ui.text_edit_singleline(&mut self.filter.search).changed();
                self.filter.dirty |= GenericMultiSelect::from_enum(
                    &mut self.filter.formats,
                    "nds_fs_tree_filter_format_multiselect",
                )
                .ui(ui)
                .changed();
            });

            self.filter.dirty |= ui
                .checkbox(
                    &mut self.filter.include_empty_dirs,
                    "Show empty directories",
                )
                .changed();
        })
        .response
    }
}
