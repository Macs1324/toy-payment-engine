use thiserror::Error;

use crate::transaction::{Transaction, TransactionId};

#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("Overwriting an existing transaction\n ID: {id} - {data:?}")]
    OverwritingExistingTransaction {
        id: TransactionId,
        data: Transaction,
    },
}

pub type Result<T> = std::result::Result<T, PaymentError>;
