use thiserror::Error;

use crate::event::TransactionEvent;

#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("Overwriting an existing transaction\n {tx:?}")]
    OverwritingExistingTransaction { tx: TransactionEvent },
}

pub type Result<T> = std::result::Result<T, PaymentError>;
