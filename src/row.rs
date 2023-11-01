pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl Row {
    pub fn new(id: u32, username: String, email: String) -> Row {
        Row {
            id,
            username,
            email,
        }
    }
}