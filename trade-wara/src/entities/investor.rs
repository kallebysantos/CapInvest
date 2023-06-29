use std::collections::HashMap;

struct Investor {
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

}

