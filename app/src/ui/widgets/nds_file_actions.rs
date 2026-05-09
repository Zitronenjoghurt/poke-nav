use crate::utils::file_saver::FileSaver;
use egui::Response;
use poke_nav::codec::nds::fs::file::NdsFile;

pub struct NdsFileActions<'a> {
    file: &'a NdsFile,
}

impl<'a> NdsFileActions<'a> {
    pub fn new(file: &'a NdsFile) -> Self {
        Self { file }
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
                let data = self.file.data.raw().unwrap();
                FileSaver::new()
                    .file_name(&name)
                    .title("Dump File")
                    .dispatch(data);
            }
        })
        .response
    }
}
