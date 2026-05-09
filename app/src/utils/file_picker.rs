use crate::utils::task::Task;
use crate::utils::NativeOnlySend;
use anyhow::Context;
use poke_nav::codec::common::rom::Rom;
use rfd::AsyncFileDialog;
use std::io::Cursor;

pub struct PickedFile {
    pub name: String,
    pub data: Vec<u8>,
}

pub struct FilePicker {
    dialog: AsyncFileDialog,
}

impl FilePicker {
    pub fn new() -> Self {
        Self {
            dialog: AsyncFileDialog::new(),
        }
    }

    pub fn filter(mut self, name: &str, exts: &[&str]) -> Self {
        self.dialog = self.dialog.add_filter(name, exts);
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.dialog = self.dialog.set_title(title);
        self
    }

    pub async fn pick(self) -> Option<PickedFile> {
        let file = self.dialog.pick_file().await?;
        let name = file.file_name();
        let data = file.read().await;
        Some(PickedFile { name, data })
    }

    pub fn dispatch<T, F>(self, task: &mut Task<T>, ctx: &egui::Context, f: F)
    where
        T: NativeOnlySend + 'static,
        F: FnOnce(PickedFile) -> anyhow::Result<T> + NativeOnlySend + 'static,
    {
        task.start(ctx, async move { self.pick().await.map(f) });
    }
}

// Helpers
impl FilePicker {
    pub fn pick_rom(task: &mut Task<Rom>, ui: &egui::Ui) {
        FilePicker::new()
            .filter("ROM", &["nds"])
            .title("Pick a ROM file")
            .dispatch(task, ui.ctx(), |file| {
                let mut cursor = Cursor::new(file.data);
                Rom::read(&mut cursor).context("Failed to parse ROM")
            });
    }
}
