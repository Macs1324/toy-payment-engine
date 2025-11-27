use std::collections::HashMap;

use crate::{
    ClientId,
    client::ClientData,
    error::Result,
    event::{Event, TransactionEvent, TransactionId},
};
#[derive(Debug)]
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

    pub fn apply_event(&mut self, event: &Event) -> Result<()> {
        let client_id = event.get_client();
        let client_data = self
            .clients
            .entry(client_id)
            .or_insert_with(ClientData::new);

        client_data.apply_event(event, &self.history)?;

        if let Event::Transaction(tx) = event {
            self.history.append(tx)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TransactionHistory {
    // Storing it as a hashmap since it doesn't look like there is a need to keep the transactions
    // chronologically "sorted", instead we just need the distinction whether or not a transaction
    // has already happened. And lookups in a hashmap are faster than just doing linear search
    lookup: HashMap<TransactionId, TransactionEvent>,
}

impl TransactionHistory {
    pub fn new() -> Self {
        Self {
            lookup: HashMap::new(),
        }
    }

    pub fn retrieve(&self, transaction_id: TransactionId) -> Option<&TransactionEvent> {
        self.lookup.get(&transaction_id)
    }

    pub fn append(&mut self, tx: &TransactionEvent) -> Result<()> {
        let transaction_id = tx.get_id();

        if let Some(existing_transaction) = self.lookup.insert(transaction_id, tx.clone()) {
            // Something went wrong and we're ending up overwriting an existing transaction in the
            // history
            return Err(crate::error::PaymentError::OverwritingExistingTransaction {
                tx: existing_transaction,
            });
        }
        Ok(())
    }
}
