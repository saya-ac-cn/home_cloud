use serde::{Deserialize, Serialize};
use crate::entity::domain::Log;
use crate::entity::dto::log::LogDTO;
use crate::util::date_time::DateTimeUtil;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogTypeVO{
    pub category:Option<String>,
    pub describe:Option<String>,
}
impl_field_name_method!(LogTypeVO{category,describe});