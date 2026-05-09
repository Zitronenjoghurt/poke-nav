use crate::ui::state::UiState;
use crate::utils::task::Task;
use egui::{Ui, WidgetText};
use poke_nav::codec::common::rom::Rom;
use strum_macros::EnumIter;

mod file_explorer;
mod map;
mod settings;

pub struct TabViewer<'a> {
    pub state: &'a mut UiState,
    pub loaded_rom: &'a mut Task<Rom>,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::FileExplorer => file_explorer::show(self, ui),
            Tab::Map => map::show(self, ui),
            Tab::Settings => settings::show(self, ui),
        }
    }

    fn is_closeable(&self, tab: &Self::Tab) -> bool {
        tab.closable()
    }

    fn allowed_in_windows(&self, tab: &mut Self::Tab) -> bool {
        tab.closable()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, EnumIter)]
pub enum Tab {
    FileExplorer,
    Map,
    Settings,
}

impl Tab {
    pub fn title(&self) -> &'static str {
        match self {
            Tab::FileExplorer => "File Explorer",
            Tab::Map => "Map",
            Tab::Settings => "Settings",
        }
    }

    pub fn closable(&self) -> bool {
        !matches!(self, Tab::Map)
    }
}
