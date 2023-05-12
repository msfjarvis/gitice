use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PersistableRepo {
    pub remote_url: String,
    pub head: String,
}
