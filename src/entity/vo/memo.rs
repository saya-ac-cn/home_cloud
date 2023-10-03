use crate::entity::table::Memo;
use serde::{Deserialize, Serialize};

/// 便笺展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoVO {
    pub id: Option<u64>,
    pub organize: Option<u64>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

impl From<Memo> for MemoVO {
    fn from(arg: Memo) -> Self {
        Self {
            id: arg.id,
            organize: arg.organize,
            source: arg.source,
            title: arg.title,
            content: arg.content,
            create_time: arg.create_time,
            update_time: arg.update_time,
        }
    }
}
