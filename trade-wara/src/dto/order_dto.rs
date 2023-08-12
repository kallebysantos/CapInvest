use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "order_type")]
pub enum IncomingOrderDTO<'a> {
    #[serde(borrow)]
    Buy(OrderDTO<'a>),
    Sell(OrderDTO<'a>),
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct OrderDTO<'a> {
    pub id: &'a str,
    pub investor_id: &'a str,
    pub investor_name: &'a str,
    pub asset_id: &'a str,
    pub price: f32,
    pub quantity: u32,
}
