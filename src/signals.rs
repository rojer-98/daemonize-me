use crate::Daemon;
use nix::sys::signal::{signal, SigHandler, Signal};
use std::collections::HashMap;

pub struct SignalEvent {
    signal: Signal,
    context: Daemon,
}

pub struct SignalingContext {
    handlers: HashMap<String, Box<dyn Fn(SignalEvent)>>,
}

impl SignalingContext {
    pub fn new() -> Self {
        SignalingContext {
            handlers: HashMap::new(),
        }
    }

    /// Register a handle for a specific signal
    ///
    /// parameters:
    ///
    /// * target: the signal to be handled by this function
    /// * handler: the function that will handle the signal passed to it
    /// in the form of an SignalEvent struct
    pub fn register_handler<F>(
        mut self,
        target: Signal,
        handler: &'static dyn Fn(SignalEvent),
    ) -> Self
    where
        F: Fn(SignalEvent),
    {
        self.handlers
            .insert(format!("{:?}", target), Box::new(handler));
        self
    }
}
