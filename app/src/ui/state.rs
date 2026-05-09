use poke_nav::codec::nds::fs::path::NdsPath;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct UiState {
    #[serde(skip, default)]
    pub selected_file_explorer_path: Option<NdsPath>,
}
