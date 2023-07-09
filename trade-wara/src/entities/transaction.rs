use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::order::{Buy, OrderTransition, Sell};

#[derive(Debug)]
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
}
