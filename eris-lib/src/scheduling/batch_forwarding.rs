use std::{
    num::NonZeroUsize,
    ops::{Deref, DerefMut},
    time::Duration,
};

use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
    time::timeout,
};

/// Starts a background task which receives a stream of Ts over a channel
/// and produces a stream of non-empty Vec<T>s over a different channel.
/// Batches will be collected until either max_size items have been
/// received, or until batch_duration has elapsed since the first item
/// was received.
/// If the input channel is closed, the last batch will be forwarded
/// and the loop will gracefully exit.
/// However, if the output channel is closed, the last batch will be
/// dropped.
pub async fn batch_forward<T, TX, RX>(
    mut input: RX,
    output: TX,
    batch_duration: Duration,
    max_size: NonZeroUsize,
) where
    RX: DerefMut<Target = UnboundedReceiver<T>>,
    TX: Deref<Target = UnboundedSender<Vec<T>>>,
{
    // Need NonZeroUsize to assert not zero in signature
    let max_size = max_size.get();

    // If max_size is 1, we don't need to do anything with timing
    if max_size == 1 {
        while let Some(item) = input.recv().await {
            if output.send(vec![item]).is_err() {
                tracing::warn!("Batch receiver closed handle, batch dropped");
                return;
            }
        }
        tracing::info!("Batch forwarding input handle closed, exiting batch forward");
        return;
    }

    // Wait for the first item
    while let Some(item) = input.recv().await {
        let mut batch = Vec::with_capacity(max_size);
        batch.push(item);

        match timeout(batch_duration, async {
            while batch.len() < max_size {
                if let Some(item) = input.recv().await {
                    batch.push(item);
                } else {
                    break;
                }
            }
        })
        .await
        {
            Ok(_) => {
                tracing::trace!("Sending full batch");
            }
            Err(_) => {
                tracing::trace!("Batch timed out, sending partial batch");
            }
        }

        if output.send(batch).is_err() {
            tracing::warn!("Batch receiver closed handle, batch dropped");
            return;
        }
    }
    tracing::info!("Batch forwarding input handle closed, exiting batch forward");
}
