use crate::ui::state::UiState;
use crate::utils::task::Task;
use egui::{Ui, WidgetText};
use poke_nav::rom::Rom;
use strum_macros::EnumIter;

mod file_explorer;
mod file_info;
mod map;
mod rom_info;
mod settings;

pub struct TabViewer<'a> {
    pub state: &'a mut UiState,
    pub loaded_rom: &'a mut Task<Rom>,
    pub toasts: &'a mut egui_notify::Toasts,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        match tab {
            Tab::FileExplorer => file_explorer::show(self, ui),
            Tab::FileInfo => file_info::show(self, ui),
            Tab::Map => map::show(self, ui),
            Tab::RomInfo => rom_info::show(self, ui),
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
    FileInfo,
    Map,
    RomInfo,
    Settings,
}

impl Tab {
    pub fn title(&self) -> &'static str {
        match self {
            Tab::FileExplorer => "File Explorer",
            Tab::FileInfo => "File Info",
            Tab::Map => "Map",
            Tab::RomInfo => "Rom Info",
            Tab::Settings => "Settings",
        }
    }

    pub fn closable(&self) -> bool {
        !matches!(self, Tab::Map)
    }
}
