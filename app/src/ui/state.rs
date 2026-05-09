use poke_nav::codec::nds::fs::path::NdsPath;

pub mod settings;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct UiState {
    pub settings: settings::Settings,
    #[serde(skip, default)]
    pub selected_file_explorer_path: Option<NdsPath>,
}

impl UiState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.settings.apply(ctx);
    }
}
