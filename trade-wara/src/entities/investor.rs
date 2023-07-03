use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Investor {
    id: String,
    name: String,
    assets: HashMap<String, u32>,
}

impl Investor {
    pub fn new(
        id: String,
        name: String,
        assets: Vec<(String, u32)>,
    ) -> Investor {
        Investor {
            id,
            name,
            assets: HashMap::from_iter(assets),
        }
    }

    pub fn increment_asset(&mut self, asset_id: String, quantity: u32) {
        self.assets
            .entry(asset_id)
            .and_modify(|current| *current += quantity)
            .or_insert(quantity);
    }

    pub fn decrement_asset(
        &mut self,
        asset_id: String,
        quantity: u32,
    ) -> Result<(), String> {
        let Some(asset) = self.assets.get_mut(&asset_id)  else {
            return  Err("Asset not found".into());
        };

        if *asset < quantity {
            return Err("Out range quantity".into());
        }

        *asset -= quantity;

        Ok(())
    }

    pub(super) fn assets(&self) -> &HashMap<String, u32> {
        &self.assets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_investor_assets() {
        let mut investor = Investor::new(
            "123".into(),
            "Joe".into(),
            vec![("HGLG11".into(), 10)],
        );

        investor.increment_asset("MXRF11".into(), 20);
        investor.increment_asset("HGLG11".into(), 5);

        assert_eq!(investor.assets["MXRF11"], 20);
        assert_eq!(investor.assets["HGLG11"], 15);
        assert_eq!(investor.assets.len(), 2);
    }

    #[test]
    fn decrement_investor_assets() {
        let mut investor = Investor::new(
            "123".into(),
            "Joe".into(),
            vec![("HGLG11".into(), 10)],
        );

        assert_eq!(
            Err("Asset not found".into()),
            investor.decrement_asset("MXRF11".into(), 20)
        );

        assert_eq!(Ok(()), investor.decrement_asset("HGLG11".into(), 7));
        assert_eq!(investor.assets["HGLG11"], 3);

        assert_eq!(
            Err("Out range quantity".into()),
            investor.decrement_asset("HGLG11".into(), 6)
        );

        assert_eq!(investor.assets["HGLG11"], 3);
        assert_eq!(investor.assets.len(), 1);
    }
}
