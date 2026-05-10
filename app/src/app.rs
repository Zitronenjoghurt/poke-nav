use crate::ui::icons;
use crate::ui::state::UiState;
use crate::ui::tabs::{Tab, TabViewer};
use crate::utils::file_picker::FilePicker;
use crate::utils::task::Task;
use eframe::{CreationContext, Frame, Storage};
use egui::{CentralPanel, FontDefinitions, Panel, Ui};
use egui_dock::DockState;
use egui_notify::Toasts;
use poke_nav::rom::Rom;
use strum::IntoEnumIterator;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PokeNav {
    dock: DockState<Tab>,
    ui_state: UiState,
    #[serde(skip, default)]
    loaded_rom: Task<Rom>,
    #[serde(skip, default)]
    toasts: Toasts,
}

impl Default for PokeNav {
    fn default() -> Self {
        Self {
            dock: DockState::new(vec![Tab::FileExplorer]),
            ui_state: Default::default(),
            loaded_rom: Default::default(),
            toasts: Default::default(),
        }
    }
}

impl PokeNav {
    pub fn new(cc: &CreationContext) -> Self {
        Self::setup_fonts(&cc.egui_ctx);
        cc.storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }

    fn setup_fonts(ctx: &egui::Context) {
        let mut fonts = FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }
}

impl eframe::App for PokeNav {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        self.render(ui);
        self.ui_state.update(ui);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

// Rendering
impl PokeNav {
    fn render(&mut self, ui: &mut Ui) {
        self.show_top_bar(ui);

        CentralPanel::default().show_inside(ui, |ui| {
            let mut viewer = TabViewer {
                state: &mut self.ui_state,
                loaded_rom: &mut self.loaded_rom,
                toasts: &mut self.toasts,
            };
            egui_dock::DockArea::new(&mut self.dock)
                .style(egui_dock::Style::from_egui(ui.style().as_ref()))
                .show_leaf_collapse_buttons(false)
                .show_leaf_close_all_buttons(false)
                .show_inside(ui, &mut viewer);
        });
    }
}

// Helpers
impl PokeNav {
    fn open_tab(&mut self, tab: Tab) {
        if let Some(path) = self.dock.find_tab(&tab) {
            let _ = self.dock.set_active_tab(path);
            return;
        }

        let mut tools_node = None;
        for t in Tab::iter().filter(|t| *t != Tab::Map) {
            if let Some(path) = self.dock.find_tab(&t) {
                tools_node = Some(path);
                break;
            }
        }

        let map_loc = self.dock.find_tab(&Tab::Map);
        if let Some(path) = tools_node {
            self.dock.set_focused_node_and_surface(path.node_path());
            self.dock.main_surface_mut().push_to_focused_leaf(tab);
        } else if let Some(map_path) = map_loc {
            self.dock
                .main_surface_mut()
                .split_right(map_path.node, 0.6, vec![tab]);
        } else {
            self.dock.main_surface_mut().push_to_focused_leaf(tab);
        }
    }
}

// Top bar
impl PokeNav {
    fn show_top_bar(&mut self, ui: &mut Ui) {
        Panel::top("top_bar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Poké-Nav");

                ui.separator();

                if ui
                    .button(icons::FOLDER_OPEN)
                    .on_hover_text("Load ROM")
                    .clicked()
                {
                    FilePicker::pick_rom(&mut self.loaded_rom, ui);
                }

                if ui.button(icons::GEAR).on_hover_text("Settings").clicked() {
                    self.open_tab(Tab::Settings);
                }

                ui.menu_button(icons::FILES, |ui| {
                    if ui.button("File Explorer").clicked() {
                        self.open_tab(Tab::FileExplorer);
                    }
                    if ui.button("File Info").clicked() {
                        self.open_tab(Tab::FileInfo);
                    }
                })
                .response
                .on_hover_text("Files");

                if ui.button(icons::INFO).on_hover_text("Rom Info").clicked() {
                    self.open_tab(Tab::RomInfo);
                }

                if ui.button("Map header").clicked()
                    && let Some(rom) = self.loaded_rom.get()
                    && let Rom::Nds(nds) = rom
                {
                    nds.find_hgss_map_header_table_offset();
                }
            });
        });
    }
}
