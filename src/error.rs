use thiserror::Error;

use crate::event::TransactionEvent;

#[derive(Debug, Error)]
pub enum PaymentError {
    #[error("Usage: cargo run -- <filename>")]
    InvalidCliArguments,
    #[error("Overwriting an existing transaction\n {tx:?}")]
    OverwritingExistingTransaction { tx: TransactionEvent },
    #[error("Could not parse event type: {0}")]
    UnrecognizedEventType(String),
    #[error("No monetary amount was specified for a transaction event")]
    NoAmountSpecifiedForTransaction,
}

pub type Result<T> = std::result::Result<T, PaymentError>;
