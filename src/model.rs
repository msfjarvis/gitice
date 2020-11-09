use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct PersistableRepo {
    pub(crate) remote_url: String,
    pub(crate) head: String,
}
