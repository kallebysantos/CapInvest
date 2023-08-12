use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

use serde::Deserialize;

use crate::{
    dto::order_dto::IncomingOrderDTO, entities::asset::Asset,
    entities::investor::Investor, ComparableFloat,
};

pub trait OrderItem: Sync + Send {
    fn resolve_type(&self) -> OrderResolution;

    fn asset_id(&self) -> &str;
}
pub trait OrderType: Sync + Send + PartialEq + Eq {}
pub trait OrderState: Sync + Send + PartialEq + Eq {}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Buy;
impl OrderType for Buy {}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Sell;
impl OrderType for Sell {}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Open;
impl OrderState for Open {}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Closed;
impl OrderState for Closed {}

#[derive(Debug, PartialEq, Eq)]
pub struct Order<T: OrderType, S: OrderState> {
    id: String,
    price: ComparableFloat,
    shares: u32,
    pending_shares: u32,
    asset: Asset,
    investor: Investor,
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

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "order_type", from = "IncomingOrderDTO")]
pub enum OrderResolution {
    Sell(OrderTransition<Sell>),
    Buy(OrderTransition<Buy>),
}

impl<T: OrderType, S: OrderState> Order<T, S> {
    pub fn new(
        asset: Asset,
        investor: Investor,
        id: &str,
        price: f32,
        shares: u32,
    ) -> Order<T, S> {
        Order::<T, S> {
            id: id.into(),
            price: price.into(),
            shares,
            pending_shares: shares,
            asset,
            investor,
            state: PhantomData,
            order_type: PhantomData,
        }
    }

    pub(crate) fn copy<TState: OrderState>(&self) -> Order<T, TState> {
        Order::<T, TState> {
            id: self.id.to_owned(),
            price: self.price.to_owned(),
            shares: self.shares.to_owned(),
            pending_shares: self.pending_shares,
            asset: self.asset.to_owned(),
            investor: self.investor.to_owned(),
            state: PhantomData,
            order_type: PhantomData,
        }
    }

    fn change_type<TType: OrderType>(&self) -> Order<TType, S> {
        Order::<TType, S> {
            id: self.id.to_owned(),
            price: self.price.to_owned(),
            shares: self.shares.to_owned(),
            pending_shares: self.pending_shares,
            asset: self.asset.to_owned(),
            investor: self.investor.to_owned(),
            state: PhantomData,
            order_type: PhantomData,
        }
    }

    pub fn check_order(&self) -> OrderTransition<T> {
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

    pub fn investor(&self) -> &Investor {
        &self.investor
    }

    pub fn asset(&self) -> &Asset {
        &self.asset
    }
}

impl<T: OrderType> Order<T, Open> {
    pub fn pending_shares(&self) -> &u32 {
        &self.pending_shares
    }
}

impl Order<Buy, Open> {
    pub fn buy(
        &mut self,
        share_count: u32,
    ) -> Result<OrderTransition<Buy>, OrderError> {
        if self.pending_shares < share_count {
            return Err(OrderError::OutRangeShareCount);
        }

        self.investor.increment_asset(self.asset.id(), share_count);

        self.pending_shares -= share_count;

        Ok(self.check_order())
    }
}

impl Order<Sell, Open> {
    pub fn sell(
        &mut self,
        share_count: u32,
    ) -> Result<OrderTransition<Sell>, OrderError> {
        if self.pending_shares < share_count {
            return Err(OrderError::OutRangeShareCount);
        }

        if let Err(_) =
            self.investor.decrement_asset(self.asset.id(), share_count)
        {
            return Err(OrderError::OutRangeShareCount);
        }

        self.pending_shares -= share_count;

        Ok(self.check_order())
    }
}

impl<T: OrderType + 'static, S: OrderState> OrderItem for Order<T, S> {
    fn resolve_type(&self) -> OrderResolution {
        if TypeId::of::<PhantomData<Buy>>() == self.order_type.type_id() {
            return OrderResolution::Buy(self.change_type().check_order());
        }

        if TypeId::of::<PhantomData<Sell>>() == self.order_type.type_id() {
            return OrderResolution::Sell(self.change_type().check_order());
        }

        panic!("Invalid order type: {:?}", self.order_type);
    }

    fn asset_id(&self) -> &str {
        self.asset.id()
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

impl<T: OrderType + 'static> Into<Box<dyn OrderItem>> for OrderTransition<T> {
    fn into(self) -> Box<dyn OrderItem> {
        match self {
            OrderTransition::Open(order) => Box::new(order),
            OrderTransition::Closed(order) => Box::new(order),
        }
    }
}

impl<'a> From<IncomingOrderDTO<'a>> for OrderResolution {
    fn from(value: IncomingOrderDTO) -> OrderResolution {
        match value {
            IncomingOrderDTO::Buy(order) => {
                OrderResolution::Buy(OrderTransition::Open(Order::new(
                    Asset::new(order.asset_id),
                    Investor::new(
                        order.investor_id,
                        order.investor_name,
                        vec![],
                    ),
                    order.id,
                    order.price,
                    order.quantity,
                )))
            }

            IncomingOrderDTO::Sell(order) => {
                OrderResolution::Sell(OrderTransition::Open(Order::new(
                    Asset::new(order.asset_id),
                    Investor::new(
                        order.investor_id,
                        order.investor_name,
                        vec![(order.asset_id.into(), order.quantity)],
                    ),
                    order.id,
                    order.price,
                    order.quantity,
                )))
            }
        }
    }
}

impl Into<Box<dyn OrderItem>> for OrderResolution {
    fn into(self) -> Box<dyn OrderItem> {
        match self {
            OrderResolution::Sell(order) => order.into(),
            OrderResolution::Buy(order) => order.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, collections::BinaryHeap};

    use super::*;

    #[test]
    fn cmp_order() {
        let asset = Asset::new("HGLG11");
        let investor = Investor::new("123", "Joe", vec![("HGLG11".into(), 10)]);

        let high_order = Order::<Sell, Open>::new(
            asset.to_owned(),
            investor.to_owned(),
            "321",
            7.0,
            3,
        );
        let small_order = Order::<Sell, Open>::new(
            asset.to_owned(),
            investor.to_owned(),
            "123",
            3.0,
            5,
        );

        assert_eq!(Ordering::Greater, high_order.cmp(&small_order));
        assert_eq!(Ordering::Equal, high_order.cmp(&high_order));
        assert_eq!(Ordering::Less, small_order.cmp(&high_order));
    }

    #[test]
    fn heap_orders() {
        let asset = Asset::new("HGLG11");
        let investor = Investor::new("123", "Joe", vec![("HGLG11".into(), 10)]);

        let mut heap = BinaryHeap::<Order<Sell, Open>>::new();
        heap.push(Order::new(
            asset.to_owned(),
            investor.to_owned(),
            "1",
            5.0,
            5,
        ));
        heap.push(Order::new(
            asset.to_owned(),
            investor.to_owned(),
            "2",
            7.0,
            3,
        ));
        heap.push(Order::new(
            asset.to_owned(),
            investor.to_owned(),
            "3",
            3.75,
            100,
        ));

        assert_eq!(3, heap.len());
        assert_eq!(7.0, *heap.pop().unwrap().price);
        assert_eq!(5.0, *heap.pop().unwrap().price);
        assert_eq!(3.75, *heap.pop().unwrap().price);
    }

    #[test]
    fn check_order_state() {
        let asset = Asset::new("HGLG11");
        let investor = Investor::new("123", "Joe", vec![("HGLG11".into(), 10)]);

        let mut order =
            Order::<Sell, Open>::new(asset, investor.to_owned(), "123", 7.0, 5);

        // "Setting pending shares to zero should close an Order"
        order.pending_shares = 0;
        assert_eq!(OrderTransition::Closed(order.copy()), order.check_order());

        // "Setting pending shares greater than zero should keep Order open"
        order.pending_shares = 5;
        assert_eq!(OrderTransition::Open(order.copy()), order.check_order());
    }

    #[test]
    fn check_sell() {
        let asset = Asset::new("HGLG11");
        let investor = Investor::new("123", "Joe", vec![("HGLG11".into(), 10)]);

        let mut order =
            Order::<Sell, Open>::new(asset, investor.to_owned(), "123", 7.0, 5);

        // "Selling more than it owns should return an Err"
        assert_eq!(Err(OrderError::OutRangeShareCount), order.sell(10));

        // "Selling less than it owns should be Ok and keep it open"
        let sell_partial = order.sell(3);
        assert_eq!(Ok(OrderTransition::Open(order.copy())), sell_partial);
        assert_eq!(7, order.investor.assets()["HGLG11"]);

        // "Selling all pending shares should be Ok and change to closed"
        let sell_remain = order.sell(2);
        assert_eq!(Ok(OrderTransition::Closed(order.copy())), sell_remain);
        assert_eq!(5, order.investor.assets()["HGLG11"]);
    }

    #[test]
    fn check_buy() {
        let asset = Asset::new("HGLG11");
        let investor = Investor::new("123", "Joe", vec![("HGLG11".into(), 10)]);

        let mut order =
            Order::<Buy, Open>::new(asset, investor.to_owned(), "123", 7.0, 5);

        // "Buy more than it needs should return an Err"
        assert_eq!(Err(OrderError::OutRangeShareCount), order.buy(10));

        // "Buy less than it needs should be Ok and keep it open"
        let buy_partial = order.buy(3);
        assert_eq!(Ok(OrderTransition::Open(order.copy())), buy_partial);
        assert_eq!(13, order.investor.assets()["HGLG11"]);

        // "Buy all pending shares should be Ok and change to closed"
        let buy_remain = order.buy(2);
        assert_eq!(Ok(OrderTransition::Closed(order.copy())), buy_remain);
        assert_eq!(15, order.investor.assets()["HGLG11"]);
    }
}
