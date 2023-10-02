use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogTypeDTO {
    pub category: Option<String>,
    pub detail: Option<String>,
}
