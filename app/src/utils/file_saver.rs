use rfd::AsyncFileDialog;

pub struct FileSaver {
    dialog: AsyncFileDialog,
    name: String,
}

impl FileSaver {
    pub fn new() -> Self {
        Self {
            dialog: AsyncFileDialog::new(),
            name: String::from("download"),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.dialog = self.dialog.set_title(title);
        self
    }

    pub fn file_name(mut self, name: &str) -> Self {
        self.dialog = self.dialog.set_file_name(name);
        self.name = name.to_string();
        self
    }

    pub fn dispatch(self, data: Vec<u8>) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            crate::spawn(async move {
                if let Some(handle) = self.dialog.save_file().await {
                    let _ = handle.write(&data).await;
                }
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            trigger_download(&self.name, &data);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn trigger_download(filename: &str, data: &[u8]) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    let uint8 = js_sys::Uint8Array::from(data);
    let array = js_sys::Array::new();
    array.push(&uint8.buffer());

    let mut opts = BlobPropertyBag::new();
    opts.type_("application/octet-stream");

    let blob = Blob::new_with_buffer_source_sequence_and_options(&array, &opts).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let window = web_sys::window().unwrap();
    let doc = window.document().unwrap();
    let a = doc
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();

    a.set_href(&url);
    a.set_download(filename);
    a.style().set_property("display", "none").ok();
    doc.body().unwrap().append_child(&a).ok();
    a.click();
    doc.body().unwrap().remove_child(&a).ok();
    Url::revoke_object_url(&url).ok();
}
