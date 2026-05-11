use crate::event::AppEvent;
use crate::ui::widgets::nds_fs_tree::NdsFsTreeState;
use poke_nav::platform::nds::fs::path::NdsPath;

pub mod settings;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct UiState {
    pub settings: settings::Settings,
    #[serde(skip, default)]
    pub selected_file_explorer_path: Option<NdsPath>,
    pub nds_fs_tree: NdsFsTreeState,
}

impl UiState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.settings.apply(ctx);

        if AppEvent::RomLoaded.fired(ctx) {
            self.nds_fs_tree.filter.dirty = true;
        }
    }
}
