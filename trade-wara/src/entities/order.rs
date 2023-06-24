use std::marker::PhantomData;

use macro_rules_attribute::{derive, derive_alias};
use trade_wara::ComparableFloat;

derive_alias! {
    #[derive(Ord!)] = #[derive(PartialEq, Eq, Ord, PartialOrd)];
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
