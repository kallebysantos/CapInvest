pub struct Asset {
    id: String,
    name: String,
}

impl Asset {
    pub fn new(id: String, name: String) -> Self {
        Asset { id, name }
    }
}
