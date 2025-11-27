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
        self.history.append(&transaction)?;

        Ok(())
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

    pub fn append(&mut self, tx: &Transaction) -> Result<()> {
        let transaction_id = if let Some(id) = tx.get_id() {
            id
        } else {
            // Skipping the transaction if it doesn't have an ID - basically operating under the assumption that
            // only withdrawals and deposits are transactions that it makes sense to store in the
            // history
            // In a real-world scenario, this wouldn't be a silent thing
            return Ok(());
        };
        if let Some(existing_transaction) = self.lookup.insert(transaction_id, tx.clone()) {
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
