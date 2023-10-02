use crate::entity::table::Abstracts;
use serde::{Deserialize, Serialize};

/// 摘要展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbstractsVO {
    pub id: Option<u64>,
    pub flag: Option<String>,
    pub tag: Option<String>,
}

impl From<Abstracts> for AbstractsVO {
    fn from(arg: Abstracts) -> Self {
        Self {
            id: arg.id,
            flag: arg.flag,
            tag: arg.tag,
        }
    }
}
