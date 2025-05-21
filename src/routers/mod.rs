pub mod cancel_task;
pub mod similar_image; // TODO:
pub mod prelude {
    pub use super::cancel_task::cancel_task;
    pub use super::similar_image::similar_image_task;
}

pub fn bridge_crossbeam_to_tokio<T: Send + 'static>(
    cross_rx: crossbeam_channel::Receiver<T>,
) -> tokio::sync::mpsc::UnboundedReceiver<T> {
    let (tx, rx): (
        tokio::sync::mpsc::UnboundedSender<T>,
        tokio::sync::mpsc::UnboundedReceiver<T>,
    ) = tokio::sync::mpsc::unbounded_channel();
    tokio::task::spawn_blocking(move || {
        for item in cross_rx.iter() {
            if tx.send(item).is_err() {
                break;
            }
        }
    });
    rx
}
