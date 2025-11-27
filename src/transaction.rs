use crate::ClientId;

pub type TransactionId = u32;

#[derive(Debug, Clone)]
pub struct FinancialTransaction {
    pub client: ClientId,
    pub id: TransactionId,
    pub amount: f32,
}

#[derive(Debug, Clone)]
pub struct OperationalTransaction {
    pub client: ClientId,
    pub target_tx: TransactionId,
}

#[derive(Debug, Clone)]
pub enum Transaction {
    Deposit(FinancialTransaction),
    Withdrawal(FinancialTransaction),
    Dispute(OperationalTransaction),
    Resolve(OperationalTransaction),
    Chargeback(OperationalTransaction),
}

// Writing this is very ugly and annoying, but it's safe and maintainable
impl Transaction {
    pub fn get_client(&self) -> ClientId {
        match self {
            Self::Deposit(tx) => tx.client,
            Self::Withdrawal(tx) => tx.client,
            Self::Dispute(tx) => tx.client,
            Self::Resolve(tx) => tx.client,
            Self::Chargeback(tx) => tx.client,
        }
    }

    pub fn get_id(&self) -> Option<TransactionId> {
        match self {
            Self::Deposit(tx) => Some(tx.id),
            Self::Withdrawal(tx) => Some(tx.id),
            _ => None,
        }
    }
}
