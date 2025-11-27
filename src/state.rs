use std::collections::HashMap;

use crate::{
    ClientId,
    client::ClientData,
    error::Result,
    transaction::{Transaction, TransactionId},
};
pub struct State {
    clients: HashMap<ClientId, ClientData>,
    history: TransactionHistory,
}

impl State {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            history: TransactionHistory::new(),
        }
    }

    pub fn apply_transaction(&mut self, transaction: &Transaction) -> Result<()> {
        let client_id = transaction.get_client();
        let client_data = self
            .clients
            .entry(client_id)
            .or_insert_with(|| ClientData::new());

        client_data.apply_transaction(transaction, &self.history)?;

        self.history.Ok(())
    }
}

pub struct TransactionHistory {
    // Storing it as a hashmap since it doesn't look like there is a need to keep the transactions
    // chronologically "sorted", instead we just need the distinction whether or not a transaction
    // has already happened. And lookups in a hashmap are faster than just doing linear search
    lookup: HashMap<TransactionId, Transaction>,
}

impl TransactionHistory {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    pub fn retrieve(&self, transaction_id: TransactionId) -> Option<&Transaction> {
        self.lookup.get(&transaction_id)
    }

    pub fn append(&mut self, transaction_id: TransactionId, data: Transaction) -> Result<()> {
        if let Some(existing_transaction) = self.lookup.insert(transaction_id, data) {
            // Something went wrong and we're ending up overwriting an existing transaction in the
            // history
            return Err(crate::error::PaymentError::OverwritingExistingTransaction {
                id: transaction_id,
                data: existing_transaction,
            });
        }
        Ok(())
    }
}
