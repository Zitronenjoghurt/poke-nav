use crate::utils::file_saver::FileSaver;
use egui::Response;
use poke_nav::platform::nds::fs::file::NdsFile;

pub struct NdsFileActions<'a> {
    file: &'a NdsFile,
    toasts: &'a mut egui_notify::Toasts,
}

impl<'a> NdsFileActions<'a> {
    pub fn new(file: &'a NdsFile, toasts: &'a mut egui_notify::Toasts) -> Self {
        Self { file, toasts }
    }
}

impl<'a> egui::Widget for NdsFileActions<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.horizontal(|ui| {
            if ui.button("Dump").clicked() {
                let name = if !self.file.name.contains(".") {
                    let ext = self
                        .file
                        .data
                        .format()
                        .map(|f| f.extension())
                        .unwrap_or("bin");
                    format!("{}.{}", self.file.name, ext)
                } else {
                    self.file.name.clone()
                };
                match self.file.data.raw() {
                    Ok(data) => {
                        FileSaver::new()
                            .file_name(&name)
                            .title("Dump File")
                            .dispatch(data);
                    }
                    Err(err) => {
                        self.toasts.error(err.to_string());
                    }
                }
            }

            if let Some(narc) = self.file.data.narc()
                && ui.button("Dump as archive").clicked()
            {
                let name = format!("{}.zip", self.file.name);
                match narc.fs.to_zip(false) {
                    Ok(data) => {
                        FileSaver::new()
                            .file_name(&name)
                            .title("Dump NARC as archive")
                            .dispatch(data);
                    }
                    Err(err) => {
                        self.toasts.error(err.to_string());
                    }
                }
            }

            if let Some(gen4map) = self.file.data.gen4map()
                && ui.button("Dump NSBMD").clicked()
            {
                let name = format!("{}.nsbmd", self.file.name);
                FileSaver::new()
                    .file_name(&name)
                    .title("Dump NSBMD file")
                    .dispatch(gen4map.nsbmd.clone());
            }
        })
        .response
    }
}
