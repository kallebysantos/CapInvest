use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    dto::transaction_dto::TransactionDTO,
    entities::order::{Buy, OrderTransition, Sell},
};

#[derive(Debug, Clone, Serialize)]
#[serde(into = "TransactionDTO")]
pub struct Transaction {
    id: String,
    buying_order: OrderTransition<Buy>,
    selling_order: OrderTransition<Sell>,
    traded_shares: u32,
    total: f32,
    traded_at: DateTime<Utc>,
}

impl Transaction {
    pub fn new(
        buying_order: OrderTransition<Buy>,
        selling_order: OrderTransition<Sell>,
        shares: u32,
        price: f32,
    ) -> Transaction {
        Transaction {
            id: Uuid::new_v4().to_string(),
            buying_order,
            selling_order,
            traded_shares: shares,
            total: shares as f32 * price,
            traded_at: Utc::now(),
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn buying_order(&self) -> &OrderTransition<Buy> {
        &self.buying_order
    }

    pub fn selling_order(&self) -> &OrderTransition<Sell> {
        &self.selling_order
    }

    pub fn traded_shares(&self) -> u32 {
        self.traded_shares
    }

    pub fn total(&self) -> f32 {
        self.total
    }

    pub fn traded_at(&self) -> DateTime<Utc> {
        self.traded_at
    }
}
