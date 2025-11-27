use crate::ClientId;

pub type TransactionId = u32;

#[derive(Debug)]
pub struct FinancialTransaction {
    pub client: ClientId
    pub tx: TransactionId,
    pub amount: f32
}

#[derive(Debug)]
pub struct OperationalTransaction {
    pub client: ClientId,
    pub target_tx: TransactionId
}

#[derive(Debug)]
pub enum Transaction {
    Deposit(FinancialTransaction),
    Withdrawal(FinancialTransaction),
    Dispute(OperationalTransaction),
    Resolve(OperationalTransaction),
}

// Writing this is very ugly and annoying, but it's safe and maintainable 
impl Transaction {
    pub fn get_client(&self) -> ClientId {
        match self {
            Self::Deposit(tx) => {
                tx.client
            },
            Self::Withdrawal(tx) => {
                tx.client
            },
            Self::Dispute(tx) => {
                tx.client
            },
            Self::Resolve(tx) => {
                tx.client
            }
        }
    }
}

