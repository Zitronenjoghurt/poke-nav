use crate::event::AppEvent;
use egui::TextureHandle;
use lru::LruCache;
use poke_nav::platform::nds::formats::nstex::{Nstex, NstexDecodeMode};
use poke_nav::platform::nds::fs::path::NdsPath;
use std::num::NonZeroUsize;

pub struct TextureCache {
    nstex: LruCache<NstexCacheKey, TextureHandle>,
}

impl Default for TextureCache {
    fn default() -> Self {
        Self {
            nstex: LruCache::new(NonZeroUsize::new(25).unwrap()),
        }
    }
}

impl TextureCache {
    pub fn update(&mut self, ctx: &egui::Context) {
        if AppEvent::RomLoaded.fired(ctx) {
            self.nstex.clear();
        }
    }

    pub fn size(&self) -> usize {
        self.nstex
            .iter()
            .map(|(_, handle)| handle.byte_size())
            .sum::<usize>()
    }

    pub fn nstex(
        &mut self,
        ctx: &egui::Context,
        path: &NdsPath,
        nstex: &Nstex,
        mode: &NstexDecodeMode,
    ) -> anyhow::Result<TextureHandle> {
        let key = NstexCacheKey::get(path, nstex, mode)?;

        if let Some(handle) = self.nstex.get(&key) {
            return Ok(handle.clone());
        }

        let rgba = nstex.decode(mode)?;
        let image = egui::ColorImage::from_rgba_unmultiplied(
            [rgba.width() as usize, rgba.height() as usize],
            rgba.as_bytes(),
        );
        let handle = ctx.load_texture(key.name(), image, egui::TextureOptions::NEAREST);

        self.nstex.put(key, handle.clone());

        Ok(handle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NstexCacheKey {
    Single {
        path: NdsPath,
        texture_name: String,
        palette_name: Option<String>,
    },
    Sheet {
        path: NdsPath,
        palette_name: Option<String>,
        columns: Option<usize>,
    },
}

impl NstexCacheKey {
    pub fn get(path: &NdsPath, nstex: &Nstex, mode: &NstexDecodeMode) -> anyhow::Result<Self> {
        match mode {
            NstexDecodeMode::Single {
                texture_index,
                palette_index,
            } => {
                let texture = nstex.get_texture(*texture_index)?;
                let palette = palette_index.map(|i| nstex.get_palette(i)).transpose()?;
                let palette_name = palette.map(|p| p.name.clone());
                Ok(Self::Single {
                    path: path.clone(),
                    texture_name: texture.name.clone(),
                    palette_name,
                })
            }
            NstexDecodeMode::Sheet {
                palette_index,
                columns,
            } => Ok(Self::Sheet {
                path: path.clone(),
                palette_name: palette_index.map(|i| {
                    nstex
                        .get_palette(i)
                        .map(|p| p.name.clone())
                        .unwrap_or_default()
                }),
                columns: *columns,
            }),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Single {
                path,
                texture_name,
                palette_name,
            } => format!(
                "nstex_single_{}_{}_{}",
                path,
                texture_name,
                palette_name.as_deref().unwrap_or("none")
            ),
            Self::Sheet {
                path,
                palette_name,
                columns,
            } => {
                format!(
                    "nstex_sheet_{}_{}_{}",
                    path,
                    palette_name.as_deref().unwrap_or("none"),
                    columns.unwrap_or(0)
                )
            }
        }
    }
}
