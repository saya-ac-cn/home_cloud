use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogTypeVO {
    pub category: Option<String>,
    pub detail: Option<String>,
}
