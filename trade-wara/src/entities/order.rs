use std::marker::PhantomData;

use macro_rules_attribute::{derive, derive_alias};
use trade_wara::ComparableFloat;

derive_alias! {
    #[derive(Ord!)] = #[derive(PartialEq, Eq, Ord, PartialOrd)];
}


#[derive(Debug, PartialEq)]
pub enum OrderTransition {
    Open(OrderInfo<OpenState>),
    Closed(OrderInfo<ClosedState>),
}

pub trait OrderState: Eq + PartialEq {}

#[derive(Debug, Ord!)]
pub struct OpenState;

impl OrderState for OpenState {}

#[derive(Debug, Ord!)]
pub struct ClosedState;

impl OrderState for ClosedState {}

#[derive(Debug, Eq, PartialEq)]
pub struct OrderInfo<S: OrderState> {
    id: String,
    price: ComparableFloat,
    shares: u32,
    pending_shares: u32,
    state: PhantomData<S>,
}

impl<S: OrderState> OrderInfo<S> {
    fn copy<T: OrderState>(&self) -> OrderInfo<T> {
        OrderInfo::<T> {
            id: self.id.to_owned(),
            price: ComparableFloat(self.price.to_owned()),
            shares: self.shares,
            pending_shares: self.pending_shares,
            state: PhantomData,
        }
    }

    fn check_order(&self) -> OrderTransition {
        if self.pending_shares > 0 {
            return OrderTransition::Open(self.copy());
        }

        OrderTransition::Closed(self.copy())
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn price(&self) -> &f32 {
        &self.price
    }
}

impl OrderInfo<OpenState> {
    pub fn new(id: String, price: f32, shares: u32) -> OrderInfo<OpenState> {
        OrderInfo::<OpenState> {
            id,
            price: price.into(),
            shares,
            pending_shares: shares,
            state: PhantomData,
        }
    }

    pub fn pending_shares(&self) -> &u32 {
        &self.pending_shares
    }
}

impl<S: OrderState> Ord for OrderInfo<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

impl<S: OrderState> PartialOrd for OrderInfo<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
#[cfg(test)]
mod tests {
    use std::{
        cmp::{Ordering, Reverse},
        collections::BinaryHeap,
    };

    use super::*;

    #[test]
    fn cmp_order() {
        let high_order = OrderInfo::new("321".to_owned(), 7.0, 3);
        let small_order = OrderInfo::new("123".to_owned(), 3.0, 5);

        assert_eq!(Ordering::Greater, high_order.cmp(&small_order));
        assert_eq!(Ordering::Equal, high_order.cmp(&high_order));
        assert_eq!(Ordering::Less, small_order.cmp(&high_order));
    }

    #[test]
    fn heap_orders() {
        let mut max_heap = BinaryHeap::new();
        max_heap.push(OrderInfo::new("2".into(), 5.0, 10));
        max_heap.push(OrderInfo::new("1".into(), 7.0, 5));
        max_heap.push(OrderInfo::new("3".into(), 3.75, 100));

        assert_eq!(3, max_heap.len());
        assert_eq!(7.0, *max_heap.pop().unwrap().price);
        assert_eq!(5.0, *max_heap.pop().unwrap().price);
        assert_eq!(3.75, *max_heap.pop().unwrap().price);

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse(OrderInfo::new("2".into(), 5.0, 10)));
        min_heap.push(Reverse(OrderInfo::new("1".into(), 7.0, 5)));
        min_heap.push(Reverse(OrderInfo::new("3".into(), 3.75, 100)));

        assert_eq!(3, min_heap.len());
        assert_eq!(3.75, *min_heap.pop().unwrap().0.price);
        assert_eq!(5.0, *min_heap.pop().unwrap().0.price);
        assert_eq!(7.0, *min_heap.pop().unwrap().0.price);
    }

    #[test]
    fn check_order_state() {
        let mut order = OrderInfo::new("123".into(), 7.0, 5);

        // "Setting pending shares to zero should close an Order"
        order.pending_shares = 0;
        assert_eq!(OrderTransition::Closed(order.copy()), order.check_order());

        // "Setting pending shares greater than zero should keep Order open"
        order.pending_shares = 5;
        assert_eq!(OrderTransition::Open(order.copy()), order.check_order());
    }

}
