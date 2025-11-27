use std::collections::HashSet;

use crate::error::Result;
use crate::state::TransactionHistory;
use crate::transaction::{Transaction, TransactionId};

pub struct ClientData {
    available: f32,
    held: f32,
    locked: bool,

    // Keeps track of ongoing disputes
    disputes: HashSet<TransactionId>,
}

impl ClientData {
    pub fn new() -> ClientData {
        Self {
            available: 0.0f32,
            held: 0.0f32,
            locked: false,
            disputes: HashSet::new(),
        }
    }

    pub fn apply_transaction(
        &mut self,
        transaction: &Transaction,
        history: &TransactionHistory,
    ) -> Result<()> {
        match transaction {
            Transaction::Deposit(tx) => {
                self.available += tx.amount;
            }
            Transaction::Withdrawal(tx) => {
                self.available -= tx.amount;
            }
            _ => todo!(),
        };
        Ok(())
    }
}
