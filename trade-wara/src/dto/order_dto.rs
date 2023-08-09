use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "order_type")]
pub enum OrderItemDTO {
    Buy(OrderDTO),
    Sell(OrderDTO),
}

#[derive(Debug, Deserialize)]
pub struct OrderDTO {
    pub id: String,
    pub investor_id: String,
    pub investor_name: String,
    pub asset_id: String,
    pub price: f32,
    pub quantity: u32,
}
