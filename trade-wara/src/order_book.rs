use std::{
    cmp::{self, Reverse},
    collections::BinaryHeap,
    sync::Arc,
};

use crate::entities::{
    order::{
        Buy, Open, Order, OrderError, OrderResolution, OrderTransition,
        OrderType, Sell,
    },
    transaction::Transaction,
};

#[derive(Debug, Default)]
pub struct OrderBook {
    asset_id: String,
    buy_orders: BinaryHeap<Order<Buy, Open>>,
    sell_orders: BinaryHeap<Reverse<Order<Sell, Open>>>,
    transactions: Vec<Arc<Transaction>>,
}

#[derive(Debug, PartialEq)]
pub enum OrderBookError {
    InvalidOrderAssetId,
    InvalidOrderState,
    NoMatchingOrderAvailable,
    MatchingError(String),
}

impl From<OrderError> for OrderBookError {
    fn from(value: OrderError) -> Self {
        match value {
            OrderError::OutRangeShareCount => {
                Self::MatchingError(format!("{:?}", value))
            }
        }
    }
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

    pub fn try_match(&mut self) -> Result<Arc<Transaction>, OrderBookError> {
        let Some(mut sell_order) = self.sell_orders.peek_mut() else {
            return Err(OrderBookError::NoMatchingOrderAvailable)
        };

        let Some(mut buy_order) = self.buy_orders.peek_mut() else {
            return Err(OrderBookError::NoMatchingOrderAvailable);
        };

        // In order to match, buy order's price should be greater or equal than sell order's price
        if buy_order.price() < sell_order.0.price() {
            return Err(OrderBookError::NoMatchingOrderAvailable);
        }

        // TODO: Get the difference between Buy and sell prices as Platform commission
        let traded_price = *buy_order.price();

        let common_shares_count = cmp::min(
            *sell_order.0.pending_shares(),
            *buy_order.pending_shares(),
        );

        let sell_order = match sell_order.0.sell(common_shares_count)? {
            OrderTransition::Closed(_) => {
                drop(sell_order);

                self.sell_orders.pop().unwrap().0.check_order()
            }
            other => other,
        };

        let buy_order = match buy_order.buy(common_shares_count)? {
            OrderTransition::Closed(_) => {
                drop(buy_order);

                self.buy_orders.pop().unwrap().check_order()
            }
            other => other,
        };

        let transaction = Arc::new(Transaction::new(
            buy_order,
            sell_order,
            common_shares_count,
            traded_price,
        ));

        self.transactions.push(transaction.clone());

        Ok(transaction.clone())
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
