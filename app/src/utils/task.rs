use crate::utils::NativeOnlySend;
use egui::{Response, Ui};
use std::sync::mpsc;

enum TaskState<T> {
    Idle,
    Pending(mpsc::Receiver<Result<T, anyhow::Error>>),
    Done(T),
    Failed(anyhow::Error),
}

pub struct Task<T> {
    state: TaskState<T>,
}

impl<T> Default for Task<T> {
    fn default() -> Self {
        Self {
            state: TaskState::Idle,
        }
    }
}

impl<T: NativeOnlySend + 'static> Task<T> {
    pub fn start(
        &mut self,
        ctx: &egui::Context,
        fut: impl Future<Output = Option<anyhow::Result<T>>> + NativeOnlySend + 'static,
    ) {
        let (tx, rx) = mpsc::channel();
        let ctx = ctx.clone();
        crate::spawn(async move {
            if let Some(result) = fut.await {
                let _ = tx.send(result);
            }
            ctx.request_repaint();
        });
        self.state = TaskState::Pending(rx);
    }

    fn sync(&mut self) {
        if let TaskState::Pending(rx) = &self.state {
            match rx.try_recv() {
                Ok(Ok(val)) => self.state = TaskState::Done(val),
                Ok(Err(err)) => self.state = TaskState::Failed(err),
                Err(mpsc::TryRecvError::Disconnected) => self.state = TaskState::Idle,
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
    }

    pub fn busy(&mut self) -> bool {
        self.sync();
        matches!(self.state, TaskState::Pending(_))
    }

    pub fn ready(&mut self) -> bool {
        self.sync();
        matches!(self.state, TaskState::Done(_))
    }

    pub fn failed(&mut self) -> bool {
        self.sync();
        matches!(self.state, TaskState::Failed(_))
    }

    pub fn take(&mut self) -> Option<T> {
        self.sync();
        match std::mem::replace(&mut self.state, TaskState::Idle) {
            TaskState::Done(val) => Some(val),
            other => {
                self.state = other;
                None
            }
        }
    }

    pub fn get(&mut self) -> Option<&T> {
        self.sync();
        match &self.state {
            TaskState::Done(val) => Some(val),
            _ => None,
        }
    }

    pub fn error(&mut self) -> Option<&anyhow::Error> {
        self.sync();
        match &self.state {
            TaskState::Failed(err) => Some(err),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
        self.state = TaskState::Idle;
    }

    pub fn show(&mut self, ui: &mut Ui) -> TaskUi<'_, T> {
        self.sync();

        if matches!(self.state, TaskState::Failed(_)) {
            let err_msg = match &self.state {
                TaskState::Failed(err) => format!("{err:#}"),
                _ => unreachable!(),
            };
            let r = ui
                .vertical(|ui| {
                    ui.colored_label(egui::Color32::RED, &err_msg);
                    if ui.button("Dismiss").clicked() {
                        self.clear();
                    }
                })
                .response;
            return TaskUi::Handled(r);
        }

        match &self.state {
            TaskState::Idle => TaskUi::Idle,
            TaskState::Pending(_) => TaskUi::Handled(ui.spinner()),
            TaskState::Done(val) => TaskUi::Done(val),
            TaskState::Failed(_) => unreachable!(),
        }
    }
}

pub enum TaskUi<'a, T> {
    Idle,
    Done(&'a T),
    Handled(Response),
}
