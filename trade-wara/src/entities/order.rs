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

#[cfg(test)]
mod tests {
    use super::*;

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
