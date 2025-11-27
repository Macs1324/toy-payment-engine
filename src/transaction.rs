use crate::ClientId;

pub type TransactionId = u32;

pub struct FinancialTransaction {
    client: ClientId
    tx: TransactionId,
    amount: f32
}

pub struct OperationalTransaction {
    client: ClientId,
    target_tx: TransactionId
}

pub enum Transaction {
    Deposit(FinancialTransaction),
    Withdrawal(FinancialTransaction),
    Dispute(OperationalTransaction),
    Resolve(OperationalTransaction),
}
