use std::collections::HashSet;

use crate::error::Result;
use crate::event::{Event, TransactionId};
use crate::state::TransactionHistory;

pub struct ClientData {
    available: f32,
    held: f32,
    locked: bool,

    // Keeps track of ongoing disputes
    disputed_transactions: HashSet<TransactionId>,
}

impl ClientData {
    pub fn new() -> ClientData {
        Self {
            available: 0.0f32,
            held: 0.0f32,
            locked: false,
            disputed_transactions: HashSet::new(),
        }
    }

    pub fn apply_event(&mut self, event: &Event, history: &TransactionHistory) -> Result<()> {
        match event {
            _ => todo!(),
        };
        Ok(())
    }
}
