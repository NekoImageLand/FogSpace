use crate::context::FogSpaceCtx;
use czkawka_core::progress_data::ProgressData;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use uuid::Uuid;

pub mod similar_image;

pub struct HandlerCtx {
    uuid: Uuid,
    space: FogSpaceCtx,
    process_sender: crossbeam_channel::Sender<ProgressData>,
}

impl HandlerCtx {
    pub fn new(
        space: FogSpaceCtx,
        process_sender: crossbeam_channel::Sender<ProgressData>,
        uuid: Uuid,
    ) -> Self {
        HandlerCtx {
            space,
            process_sender,
            uuid,
        }
    }

    pub fn task<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Arc<AtomicBool>, crossbeam_channel::Sender<ProgressData>) -> R,
    {
        let signal = Arc::new(AtomicBool::new(false));
        self.space.tasks.insert(self.uuid, signal.clone());
        let result = f(signal.clone(), self.process_sender.clone());
        self.space.tasks.remove(&self.uuid);
        result
    }
}
