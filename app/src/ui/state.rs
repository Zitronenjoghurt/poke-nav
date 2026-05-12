use crate::event::AppEvent;
use crate::ui::widgets::nds_fs_tree::NdsFsTreeState;
use poke_nav::platform::nds::formats::nstex::NstexDecodeMode;
use poke_nav::platform::nds::fs::path::NdsPath;

pub mod settings;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct UiState {
    pub settings: settings::Settings,
    pub nds_fs_tree: NdsFsTreeState,
    #[serde(skip, default)]
    pub selected_file_explorer_path: Option<NdsPath>,
    #[serde(skip, default)]
    pub nstex_decode_mode: NstexDecodeMode,
}

impl UiState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.settings.apply(ctx);

        if AppEvent::RomLoaded.fired(ctx) {
            self.nds_fs_tree.filter.dirty = true;
        }
    }
}
