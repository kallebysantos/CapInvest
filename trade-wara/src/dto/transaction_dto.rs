use serde::Serialize;

use crate::entities::transaction::Transaction;

#[derive(Debug, Serialize, PartialEq)]
pub struct TransactionDTO {
    id: String,
    buying_order_id: String,
    selling_order_id: String,
    traded_shares: u32,
    total: f32,
    traded_at: String,
}

impl From<Transaction> for TransactionDTO {
    fn from(value: Transaction) -> Self {
        TransactionDTO {
            id: value.id().into(),
            buying_order_id: value.buying_order().get_order_id().into(),
            selling_order_id: value.selling_order().get_order_id().into(),
            traded_shares: value.traded_shares(),
            total: value.total(),
            traded_at: value.traded_at().to_rfc3339(),
        }
    }
}
