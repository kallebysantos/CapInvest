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

#[cfg(test)]
mod tests {
    use crate::entities::{asset::Asset, investor::Investor, order::OrderItem};

    use super::*;

    #[test]
    fn append_orders() {
        let asset_1 = Asset::new("HGLG11".into());
        let asset_2 = Asset::new("WRONG11".into());

        let mut book = OrderBook::new(asset_1.id().to_owned());

        let investor = Investor::new(
            "123".into(),
            "Foo".into(),
            vec![(asset_1.id().to_owned(), 10)],
        );

        let sell_order = Order::<Sell, Open>::new(
            asset_1.to_owned(),
            investor.to_owned(),
            "A".into(),
            5.0,
            10,
        );

        let sell_order_2 = Order::<Sell, Open>::new(
            asset_1.to_owned(),
            investor.to_owned(),
            "A".into(),
            3.0,
            10,
        );

        let buy_order = Order::<Buy, Open>::new(
            asset_1.to_owned(),
            investor.to_owned(),
            "B".into(),
            3.0,
            10,
        );

        let buy_order_2 = Order::<Buy, Open>::new(
            asset_1.to_owned(),
            investor.to_owned(),
            "C".into(),
            5.0,
            10,
        );

        let wrong_order = Order::<Buy, Open>::new(
            asset_2.to_owned(),
            investor.to_owned(),
            "D".into(),
            1.0,
            10,
        );

        assert!(book.append(sell_order.resolve_type()).is_ok());
        assert!(book.append(sell_order_2.resolve_type()).is_ok());
        assert!(book.append(buy_order.resolve_type()).is_ok());
        assert!(book.append(buy_order_2.resolve_type()).is_ok());

        assert_eq!(
            OrderBookError::InvalidOrderAssetId,
            book.append(wrong_order.resolve_type()).unwrap_err()
        );

        assert_eq!(2, book.sell_orders.len());
        assert_eq!(2, book.buy_orders.len());

        // Buy orders with increased prices should have higher priority
        assert_eq!(&buy_order_2, book.buy_orders.peek().unwrap());

        // Sell orders with lowest prices should have higher priority
        assert_eq!(&Reverse(sell_order_2), book.sell_orders.peek().unwrap());
    }

    #[test]
    fn match_orders() {
        const ORDER_PRICE: f32 = 5.0;
        const INCREASED_ORDER_PRICE: f32 = ORDER_PRICE + 0.5;

        const ORDER_QUANTITY: u32 = 10;
        const PARTIAL_QUANTITY: u32 = ORDER_QUANTITY / 2;

        let asset = Asset::new("HGLG11".into());
        let mut book = OrderBook::new(asset.id().to_owned());

        let investor_a = Investor::new(
            "123".into(),
            "Foo".into(),
            vec![(asset.id().to_owned(), 10)],
        );

        let investor_b = Investor::new("321".into(), "Bar".into(), vec![]);

        let mut order_a = Order::<Sell, Open>::new(
            asset.to_owned(),
            investor_a,
            "A".into(),
            ORDER_PRICE,
            ORDER_QUANTITY,
        );

        let mut order_b = Order::<Buy, Open>::new(
            asset.to_owned(),
            investor_b.to_owned(),
            "B".into(),
            ORDER_PRICE,
            PARTIAL_QUANTITY,
        );

        let mut order_c = Order::<Buy, Open>::new(
            asset.to_owned(),
            investor_b.to_owned(),
            "C".into(),
            INCREASED_ORDER_PRICE,
            PARTIAL_QUANTITY,
        );

        assert!(book.append(order_a.resolve_type()).is_ok());
        assert!(book.append(order_b.resolve_type()).is_ok());
        assert!(book.append(order_c.resolve_type()).is_ok());

        assert_eq!(1, book.sell_orders.len());
        assert_eq!(2, book.buy_orders.len());

        // Buy orders with increased prices should have higher priority
        assert_eq!(&order_c, book.buy_orders.peek().unwrap());

        /* 1º PARTIAL TRANSACTION */

        let partial_transaction = book.try_match().unwrap();

        const INCREASED_PARTIAL_TOTAL: f32 =
            INCREASED_ORDER_PRICE * PARTIAL_QUANTITY as f32;

        assert_eq!(PARTIAL_QUANTITY, partial_transaction.traded_shares());
        assert_eq!(INCREASED_PARTIAL_TOTAL, partial_transaction.total());

        assert!(order_a.sell(PARTIAL_QUANTITY).is_ok());
        assert!(order_c.buy(PARTIAL_QUANTITY).is_ok());

        assert_eq!(
            &OrderTransition::Open(order_a.copy()),
            partial_transaction.selling_order()
        );
        assert_eq!(
            &OrderTransition::Closed(order_c.copy()),
            partial_transaction.buying_order()
        );

        assert_eq!(1, book.transactions.len());
        assert_eq!(1, book.sell_orders.len());
        assert_eq!(1, book.buy_orders.len());
        assert_eq!(&order_b, book.buy_orders.peek().unwrap());

        /* 2º PARTIAL TRANSACTION */

        let partial_transaction = book.try_match().unwrap();
        const PARTIAL_TOTAL: f32 = ORDER_PRICE * PARTIAL_QUANTITY as f32;

        assert_eq!(PARTIAL_QUANTITY, partial_transaction.traded_shares());
        assert_eq!(PARTIAL_TOTAL, partial_transaction.total());

        assert!(order_a.sell(PARTIAL_QUANTITY).is_ok());
        assert!(order_b.buy(PARTIAL_QUANTITY).is_ok());

        assert_eq!(
            &OrderTransition::Closed(order_a.copy()),
            partial_transaction.selling_order()
        );
        assert_eq!(
            &OrderTransition::Closed(order_b.copy()),
            partial_transaction.buying_order()
        );

        assert_eq!(2, book.transactions.len());
        assert_eq!(0, book.sell_orders.len());
        assert_eq!(0, book.buy_orders.len());

        assert_eq!(
            OrderBookError::NoMatchingOrderAvailable,
            book.try_match().unwrap_err()
        )
    }
}
