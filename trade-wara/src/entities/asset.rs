#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Asset {
    id: String,
}

impl Asset {
    pub fn new(id: &str) -> Self {
        Asset { id: id.to_string() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}
