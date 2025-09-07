pub struct ContestManager {
    pub secret: String,
}

impl ContestManager {
    pub fn new() -> Self {
        Self {
            secret: "password123".to_string(),
        }
    }
}
