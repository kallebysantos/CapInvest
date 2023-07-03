#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Asset {
    id: String,
    name: String,
}

impl Asset {
    pub fn new(id: String, name: String) -> Self {
        Asset { id, name }
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
