use anyhow::anyhow;
use egui::TextureHandle;
use poke_nav::platform::nds::formats::nstex::{Nstex, NstexRef};
use std::collections::HashMap;

#[derive(Default)]
pub struct TextureCache {
    nstex: HashMap<NstexCacheKey, TextureHandle>,
}

impl TextureCache {
    pub fn nstex(
        &mut self,
        ctx: &egui::Context,
        nstex: &Nstex,
        reference: &NstexRef,
    ) -> anyhow::Result<&TextureHandle> {
        let Some((texture, palette)) = reference.resolve(nstex) else {
            return Err(anyhow!("Invalid texture reference"));
        };

        let key = NstexCacheKey {
            texture_name: texture.name.clone(),
            palette_name: palette.map(|p| p.name.clone()),
        };

        if !self.nstex.contains_key(&key) {
            let rgba = texture.decode(palette)?;
            let image = egui::ColorImage::from_rgba_unmultiplied(
                [rgba.width() as usize, rgba.height() as usize],
                rgba.as_bytes(),
            );
            let handle = ctx.load_texture(key.name(), image, egui::TextureOptions::NEAREST);
            self.nstex.insert(key.clone(), handle);
        }

        Ok(&self.nstex[&key])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NstexCacheKey {
    texture_name: String,
    palette_name: Option<String>,
}

impl NstexCacheKey {
    pub fn name(&self) -> String {
        format!(
            "nstex_{}_{}",
            self.texture_name,
            self.palette_name.as_deref().unwrap_or("none")
        )
    }
}
