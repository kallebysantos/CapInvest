use std::{cmp::Reverse, collections::BinaryHeap};

use crate::entities::{
    order::{
        Buy, Open, Order, OrderResolution, OrderTransition, OrderType, Sell,
    },
    transaction::Transaction,
};

#[derive(Debug, Default)]
pub struct OrderBook {
    asset_id: String,
    buy_orders: BinaryHeap<Order<Buy, Open>>,
    sell_orders: BinaryHeap<Reverse<Order<Sell, Open>>>,
    transactions: Vec<Transaction>,
}

#[derive(Debug, PartialEq)]
pub enum OrderBookError {
    InvalidOrderAssetId,
    InvalidOrderState,
}

impl OrderBook {
    pub fn new(asset_id: String) -> OrderBook {
        OrderBook {
            asset_id,
            ..Default::default()
        }
    }
}
