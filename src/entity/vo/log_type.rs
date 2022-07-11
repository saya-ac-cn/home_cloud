use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogTypeVO{
    pub category:Option<String>,
    pub describe:Option<String>,
}
impl_field_name_method!(LogTypeVO{category,describe});