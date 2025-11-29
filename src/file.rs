pub struct File {
    pub id: u64,
    pub name: String,
}

impl File {
    pub fn new(id: u64, name: String) -> Self {
        Self { id, name }
    }
}
