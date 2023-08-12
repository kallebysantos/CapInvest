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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_sell() {
        let json = r#"{
            "id": "a16a766e-3373-457b-965a-6aee3c145b4f",
            "investor_id": "394970b3-52aa-4dfb-8e7d-55e03251ff5c",
            "investor_name": "Joe Doe",
            "asset_id": "HGLG11",
            "price": 13.45,
            "quantity": 5,
            "order_type": "Sell"
        }"#;

        let expected_order = IncomingOrderDTO::Sell(OrderDTO {
            id: "a16a766e-3373-457b-965a-6aee3c145b4f",
            investor_id: "394970b3-52aa-4dfb-8e7d-55e03251ff5c",
            investor_name: "Joe Doe",
            asset_id: "HGLG11",
            price: 13.45,
            quantity: 5,
        });

        assert_eq!(expected_order, serde_json::from_str(json).unwrap())
    }

    #[test]
    fn deserialize_buy() {
        let json = r#"{
            "id": "a16a766e-3373-457b-965a-6aee3c145b4f",
            "investor_id": "394970b3-52aa-4dfb-8e7d-55e03251ff5c",
            "investor_name": "Joe Doe",
            "asset_id": "HGLG11",
            "price": 13.45,
            "quantity": 5,
            "order_type": "Buy"
        }"#;

        let expected_order = IncomingOrderDTO::Buy(OrderDTO {
            id: "a16a766e-3373-457b-965a-6aee3c145b4f",
            investor_id: "394970b3-52aa-4dfb-8e7d-55e03251ff5c",
            investor_name: "Joe Doe",
            asset_id: "HGLG11",
            price: 13.45,
            quantity: 5,
        });

        assert_eq!(expected_order, serde_json::from_str(json).unwrap())
    }
}
