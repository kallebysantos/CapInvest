#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Asset {
    id: String,
}

impl Asset {
    pub fn new(id: String) -> Self {
        Asset { id }
    }

    pub fn id(&self) -> &String {
        &self.id
    }
}
