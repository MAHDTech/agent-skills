//! Asynchronous event loop dispatcher consuming terminal input streams and timer ticks.

use std::time::Duration;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent};
use futures::StreamExt;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio::time::{interval, MissedTickBehavior};

/// Channel error types and re-exports.
pub mod mpsc {
    pub use tokio::sync::mpsc::*;
    /// Error types for channel operations.
    pub mod error {
        pub use std::sync::mpsc::RecvError;
        pub use tokio::sync::mpsc::error::*;
    }
}

/// Terminal input interactions, periodic timer ticks, and lifecycle events.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Keyboard key interaction.
    Key(KeyEvent),
    /// Pointer motion, click, or scroll gesture.
    Mouse(MouseEvent),
    /// Terminal window dimension adjustment as `(columns, rows)`.
    Resize(u16, u16),
    /// Periodic heartbeat timer tick for UI redraws and background refreshes.
    Tick,
    /// Explicit signal requesting application termination.
    Quit,
}

/// Asynchronous event loop dispatcher consuming terminal input streams and timer ticks.
#[derive(Debug)]
pub struct EventHandler {
    /// Channel transmitter sending typed events to the receiver.
    sender: Sender<Event>,
    /// Channel receiver polled by the main loop.
    receiver: Receiver<Event>,
    /// Background worker task handle.
    handler_task: JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new event handler spawning a background worker polling at the given tick rate.
    #[must_use]
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(256);
        let worker_sender = sender.clone();

        let handler_task = tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut tick_interval = interval(tick_rate);
            tick_interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

            loop {
                let event = tokio::select! {
                    _ = tick_interval.tick() => Event::Tick,
                    crossterm_event = reader.next() => {
                        match crossterm_event {
                            Some(Ok(CrosstermEvent::Key(key))) => {
                                if key.kind == KeyEventKind::Release {
                                    continue;
                                }
                                Event::Key(key)
                            }
                            Some(Ok(CrosstermEvent::Mouse(mouse))) => Event::Mouse(mouse),
                            Some(Ok(CrosstermEvent::Resize(w, h))) => Event::Resize(w, h),
                            Some(Err(_)) | None => break,
                            _ => continue,
                        }
                    }
                };

                if worker_sender.send(event).await.is_err() {
                    break;
                }
            }
        });

        Self {
            sender,
            receiver,
            handler_task,
        }
    }

    /// Awaits the next typed event from the channel.
    pub async fn next(&mut self) -> Result<Event, mpsc::error::RecvError> {
        self.receiver.recv().await.ok_or(mpsc::error::RecvError)
    }

    /// Returns a cloneable handle to the event channel transmitter.
    #[must_use]
    pub fn sender(&self) -> Sender<Event> {
        self.sender.clone()
    }

    /// Queues an event synchronously into the channel buffer.
    pub fn send(&self, event: Event) -> Result<(), mpsc::error::SendError<Event>> {
        self.sender
            .try_send(event)
            .map_err(|err| tokio::sync::mpsc::error::SendError(err.into_inner()))
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.handler_task.abort();
    }
}
