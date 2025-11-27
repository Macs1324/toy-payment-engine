use crate::{
    ClientId,
    error::PaymentError,
    event::{DisputeData, DisputeEvent, Event, TransactionData, TransactionEvent, TransactionId},
};

pub type EventCsvRecord = (String, ClientId, TransactionId, Option<f32>);
impl TryFrom<EventCsvRecord> for Event {
    type Error = PaymentError;
    fn try_from(value: EventCsvRecord) -> Result<Self, Self::Error> {
        let event_type = value.0;
        let client_id = value.1;
        let transaction_id = value.2;
        let amount = value.3;

        match event_type.as_str() {
            "deposit" => Ok(Event::Transaction(TransactionEvent::Deposit(
                TransactionData {
                    id: transaction_id,
                    client: client_id,
                    amount: if let Some(amount) = amount {
                        amount
                    } else {
                        return Err(PaymentError::NoAmountSpecifiedForTransaction);
                    },
                },
            ))),
            "withdrawal" => Ok(Event::Transaction(TransactionEvent::Withdrawal(
                TransactionData {
                    id: transaction_id,
                    client: client_id,
                    amount: if let Some(amount) = amount {
                        amount
                    } else {
                        return Err(PaymentError::NoAmountSpecifiedForTransaction);
                    },
                },
            ))),
            "dispute" => Ok(Event::DisputeRef(DisputeEvent::Dispute(DisputeData {
                client: client_id,
                target_tx: transaction_id,
            }))),
            "resolve" => Ok(Event::DisputeRef(DisputeEvent::Resolve(DisputeData {
                client: client_id,
                target_tx: transaction_id,
            }))),
            "chargeback" => Ok(Event::DisputeRef(DisputeEvent::Chargeback(DisputeData {
                client: client_id,
                target_tx: transaction_id,
            }))),
            _ => Err(PaymentError::UnrecognizedEventType(event_type)),
        }
    }
}
