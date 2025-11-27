use crate::ClientId;

pub type TransactionId = u32;

#[derive(Debug, Clone)]
pub struct TransactionData {
    pub client: ClientId,
    pub id: TransactionId,
    pub amount: f32,
}

#[derive(Debug, Clone)]
pub struct DisputeData {
    pub client: ClientId,
    pub target_tx: TransactionId,
}

#[derive(Debug, Clone)]
pub enum TransactionEvent {
    Deposit(TransactionData),
    Withdrawal(TransactionData),
}

#[derive(Debug, Clone)]
pub enum DisputeEvent {
    Dispute(DisputeData),
    Resolve(DisputeData),
    Chargeback(DisputeData),
}

#[derive(Debug, Clone)]
pub enum Event {
    Transaction(TransactionEvent),
    DisputeRef(DisputeEvent),
}

// Writing this is very ugly and annoying, but it's safe and maintainable
impl Event {
    pub fn get_client(&self) -> ClientId {
        match self {
            Self::Transaction(tx) => tx.get_client(),
            Self::DisputeRef(dp) => dp.get_client(),
        }
    }
}

impl TransactionEvent {
    pub fn get_client(&self) -> ClientId {
        match self {
            Self::Deposit(data) => data.client,
            Self::Withdrawal(data) => data.client,
        }
    }

    pub fn get_id(&self) -> TransactionId {
        match self {
            Self::Deposit(data) => data.id,
            Self::Withdrawal(data) => data.id,
        }
    }
}

impl DisputeEvent {
    pub fn get_client(&self) -> ClientId {
        match self {
            Self::Dispute(data) => data.client,
            Self::Resolve(data) => data.client,
            Self::Chargeback(data) => data.client,
        }
    }

    pub fn get_target(&self) -> TransactionId {
        match self {
            Self::Dispute(data) => data.target_tx,
            Self::Resolve(data) => data.target_tx,
            Self::Chargeback(data) => data.target_tx,
        }
    }
}
