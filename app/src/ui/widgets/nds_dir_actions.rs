use crate::utils::file_saver::FileSaver;
use egui::Response;
use poke_nav::codec::nds::fs::dir::NdsDirectory;
use poke_nav::codec::nds::fs::NdsFileSystem;

pub struct NdsDirActions<'a> {
    fs: &'a NdsFileSystem,
    dir: &'a NdsDirectory,
    toasts: &'a mut egui_notify::Toasts,
}

impl<'a> NdsDirActions<'a> {
    pub fn new(
        fs: &'a NdsFileSystem,
        dir: &'a NdsDirectory,
        toasts: &'a mut egui_notify::Toasts,
    ) -> Self {
        Self { fs, dir, toasts }
    }
}

impl<'a> egui::Widget for NdsDirActions<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            if ui.button("Dump as archive").clicked() {
                let dir_name = if self.dir.name.is_empty() {
                    "archive"
                } else {
                    &self.dir.name
                };
                let name = format!("{}.zip", dir_name);
                match self.fs.zip_dir_by_id(self.dir.id, false) {
                    Ok(data) => {
                        FileSaver::new()
                            .file_name(&name)
                            .title("Dump directory as archive")
                            .dispatch(data);
                    }
                    Err(err) => {
                        self.toasts.error(err.to_string());
                    }
                }
            }
        })
        .response
    }
}
