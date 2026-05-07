use crate::ui::state::UiState;
use crate::ui::tabs::{Tab, TabViewer};
use eframe::{CreationContext, Frame, Storage};
use egui::{CentralPanel, Context, FontDefinitions, Ui};
use egui_dock::DockState;
use strum::IntoEnumIterator;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PokeNav {
    dock: DockState<Tab>,
    ui_state: UiState,
}

impl Default for PokeNav {
    fn default() -> Self {
        Self {
            dock: DockState::new(vec![Tab::Map]),
            ui_state: Default::default(),
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

    fn setup_fonts(ctx: &Context) {
        let mut fonts = FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        ctx.set_fonts(fonts);
    }
}

impl eframe::App for PokeNav {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        self.render(ui);
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

// Rendering
impl PokeNav {
    fn render(&mut self, ui: &mut Ui) {
        CentralPanel::default().show_inside(ui, |ui| {
            let mut viewer = TabViewer {
                state: &mut self.ui_state,
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
