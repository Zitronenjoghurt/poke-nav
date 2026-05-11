#[derive(Default, Clone)]
pub struct EventBuffers {
    pending: Vec<AppEvent>,
    active: Vec<AppEvent>,
}

impl EventBuffers {
    pub fn flush(ctx: &egui::Context) {
        ctx.data_mut(|d| {
            let buf = d.get_temp_mut_or_default::<Self>(egui::Id::NULL);
            buf.active.clear();
            std::mem::swap(&mut buf.active, &mut buf.pending);
        });
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum AppEvent {
    RomLoaded,
}

impl AppEvent {
    pub fn send(self, ctx: &egui::Context) {
        ctx.data_mut(|d| {
            d.get_temp_mut_or_default::<EventBuffers>(egui::Id::NULL)
                .pending
                .push(self);
        });
    }

    pub fn fired(self, ctx: &egui::Context) -> bool {
        ctx.data(|d| {
            d.get_temp::<EventBuffers>(egui::Id::NULL)
                .is_some_and(|buf| buf.active.contains(&self))
        })
    }
}
