use crate::transaction::Transaction;

pub mod client;
pub mod error;
pub mod state;
pub mod transaction;

pub type ClientId = u16;

fn main() {
    let transactions: Vec<Transaction> = vec![];
}
