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

    pub fn append(
        &mut self,
        order: OrderResolution,
    ) -> Result<(), OrderBookError> {
        match order {
            OrderResolution::Sell(order) => {
                let order = self.check_is_order_valid(order)?;

                self.sell_orders.push(Reverse(order));
            }
            OrderResolution::Buy(order) => {
                let order = self.check_is_order_valid(order)?;

                self.buy_orders.push(order);
            }
        }

        Ok(())
    }

    fn check_is_order_valid<T: OrderType>(
        &self,
        order: OrderTransition<T>,
    ) -> Result<Order<T, Open>, OrderBookError> {
        match order {
            OrderTransition::Open(order) => {
                if *order.asset().id() != self.asset_id {
                    return Err(OrderBookError::InvalidOrderAssetId);
                }

                Ok(order)
            }
            OrderTransition::Closed(_) => {
                return Err(OrderBookError::InvalidOrderState);
            }
        }
    }
}
