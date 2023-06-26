use std::marker::PhantomData;

use macro_rules_attribute::{derive, derive_alias};

use crate::ComparableFloat;

derive_alias! {
    #[derive(Ord!)] = #[derive(PartialEq, Eq, Ord, PartialOrd)];
}

pub trait OrderType: PartialEq + Eq {}
pub trait OrderState: PartialEq + Eq {}

#[derive(Debug, Ord!)]
pub struct Buy;
impl OrderType for Buy {}

#[derive(Debug, Ord!)]
pub struct Sell;
impl OrderType for Sell {}

#[derive(Debug, Ord!)]
pub struct Open;
impl OrderState for Open {}

#[derive(Debug, Ord!)]
pub struct Closed;
impl OrderState for Closed {}

#[derive(Debug, Eq, PartialEq)]
pub struct Order<T: OrderType, S: OrderState> {
    id: String,
    price: ComparableFloat,
    shares: u32,
    pending_shares: u32,
    state: PhantomData<S>,
    order_type: PhantomData<T>,
}

#[derive(Debug, PartialEq)]
pub enum OrderError {
    OutRangeShareCount,
}

#[derive(Debug, PartialEq)]
pub enum OrderTransition<T: OrderType> {
    Open(Order<T, Open>),
    Closed(Order<T, Closed>),
}

impl<T: OrderType, S: OrderState> Order<T, S> {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn price(&self) -> &f32 {
        &self.price
    }

    pub fn new(id: String, price: f32, shares: u32) -> Order<T, S> {
        Order::<T, S> {
            id,
            price: price.into(),
            shares,
            pending_shares: shares,
            state: PhantomData,
            order_type: PhantomData,
        }
    }

    fn copy<TState: OrderState>(&self) -> Order<T, TState> {
        Order::<T, TState> {
            id: self.id.to_owned(),
            price: ComparableFloat(self.price.to_owned()),
            shares: self.shares,
            pending_shares: self.pending_shares,
            state: PhantomData,
            order_type: PhantomData,
        }
    }

    fn check_order(&self) -> OrderTransition<T> {
        if self.pending_shares > 0 {
            return OrderTransition::Open(self.copy());
        }

        OrderTransition::Closed(self.copy())
    }
}

impl<T: OrderType> Order<T, Open> {
    pub fn pending_shares(&self) -> &u32 {
        &self.pending_shares
    }
}

impl Order<Buy, Open> {
    pub fn buy(&mut self, share_count: u32) -> Result<OrderTransition<Buy>, OrderError> {
        if self.pending_shares < share_count {
            return Err(OrderError::OutRangeShareCount);
        }

        // todo!() increment Investor asset position Here
        self.pending_shares -= share_count;

        Ok(self.check_order())
    }
}

impl Order<Sell, Open> {
    pub fn sell(&mut self, share_count: u32) -> Result<OrderTransition<Sell>, OrderError> {
        if self.pending_shares < share_count {
            return Err(OrderError::OutRangeShareCount);
        }

        // todo!() decrement Investor asset position Here
        self.pending_shares -= share_count;

        Ok(self.check_order())
    }
}

impl<T: OrderType, S: OrderState> Ord for Order<T, S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

impl<T: OrderType, S: OrderState> PartialOrd for Order<T, S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, collections::BinaryHeap};

    use super::*;

    #[test]
    fn cmp_order() {
        let high_order = Order::<Sell, Open>::new("321".to_owned(), 7.0, 3);
        let small_order = Order::<Sell, Open>::new("123".to_owned(), 3.0, 5);

        assert_eq!(Ordering::Greater, high_order.cmp(&small_order));
        assert_eq!(Ordering::Equal, high_order.cmp(&high_order));
        assert_eq!(Ordering::Less, small_order.cmp(&high_order));
    }

    #[test]
    fn heap_orders() {
        let mut heap = BinaryHeap::<Order<Sell, Open>>::new();
        heap.push(Order::new("2".into(), 7.0, 3));
        heap.push(Order::new("1".into(), 5.0, 5));
        heap.push(Order::new("3".into(), 3.75, 100));

        assert_eq!(3, heap.len());
        assert_eq!(7.0, *heap.pop().unwrap().price);
        assert_eq!(5.0, *heap.pop().unwrap().price);
        assert_eq!(3.75, *heap.pop().unwrap().price);
    }

    #[test]
    fn check_order_state() {
        let mut order = Order::<Sell, Open>::new("123".into(), 7.0, 5);

        // "Setting pending shares to zero should close an Order"
        order.pending_shares = 0;
        assert_eq!(OrderTransition::Closed(order.copy()), order.check_order());

        // "Setting pending shares greater than zero should keep Order open"
        order.pending_shares = 5;
        assert_eq!(OrderTransition::Open(order.copy()), order.check_order());
    }

    #[test]
    fn check_sell() {
        let mut order = Order::<Sell, Open>::new("123".into(), 7.0, 5);

        // "Selling more than it owns should return an Err"
        assert_eq!(Err(OrderError::OutRangeShareCount), order.sell(10));

        // "Selling less than it owns should be Ok and keep it open"
        let sell_partial = order.sell(3);
        assert_eq!(Ok(OrderTransition::Open(order.copy())), sell_partial);

        // "Selling all pending shares should be Ok and change to closed"
        let sell_remain = order.sell(2);
        assert_eq!(Ok(OrderTransition::Closed(order.copy())), sell_remain);
    }

    #[test]
    fn check_buy() {
        let mut order = Order::<Buy, Open>::new("123".into(), 7.0, 5);

        // "Buy more than it needs should return an Err"
        assert_eq!(Err(OrderError::OutRangeShareCount), order.buy(10));

        // "Buy less than it needs should be Ok and keep it open"
        let buy_partial = order.buy(3);
        assert_eq!(Ok(OrderTransition::Open(order.copy())), buy_partial);

        // "Buy all pending shares should be Ok and change to closed"
        let buy_remain = order.buy(2);
        assert_eq!(Ok(OrderTransition::Closed(order.copy())), buy_remain);
    }
}
