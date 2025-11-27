use std::collections::HashSet;

use crate::error::Result;
use crate::event::{DisputeEvent, Event, TransactionEvent, TransactionId};
use crate::state::TransactionHistory;

#[derive(Default, Debug)]
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
        if self.locked {
            // Skipping processing if the account is locked
            // This would not be happening so silently in a real-world scenario
            return Ok(());
        }
        match event {
            Event::Transaction(tx) => match tx {
                TransactionEvent::Deposit(data) => {
                    self.available += data.amount;
                }
                TransactionEvent::Withdrawal(data) => {
                    self.available -= data.amount;
                }
            },
            Event::DisputeRef(dp) => match dp {
                DisputeEvent::Dispute(data) => {
                    if let Some(TransactionEvent::Deposit(target_tx)) =
                        history.retrieve(data.target_tx)
                    {
                        self.disputed_transactions.insert(data.target_tx);
                        self.available -= target_tx.amount;
                        self.held += target_tx.amount;
                    }
                    // Assuming that the event was issues by mistake if the transaction is not in
                    // the history
                    // Also assuming that the only transaction type that it makes sense to dispute
                    // is a deposit
                }
                DisputeEvent::Resolve(data) => {
                    if let Some(TransactionEvent::Deposit(target_tx)) =
                        history.retrieve(data.target_tx)
                    {
                        if self.disputed_transactions.contains(&data.target_tx) {
                            self.disputed_transactions.remove(&data.target_tx);
                            self.available += target_tx.amount;
                            self.held -= target_tx.amount;
                        }
                    }
                    // Assuming that the event was issues by mistake if the transaction is not in
                    // the history or not under dispute
                    //
                    // Also assuming that the only transaction type that it makes sense to dispute
                    // is a deposit
                }
                DisputeEvent::Chargeback(data) => {
                    if let Some(TransactionEvent::Deposit(target_tx)) =
                        history.retrieve(data.target_tx)
                    {
                        if self.disputed_transactions.contains(&data.target_tx) {
                            self.held -= target_tx.amount;
                            self.locked = true;
                        }
                    }
                    // Assuming that the event was issues by mistake if the transaction is not in
                    // the history or not under dispute
                }
            },
        };
        Ok(())
    }
}
