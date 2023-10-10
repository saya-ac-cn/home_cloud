use crate::entity::table::Notes;
use serde::{Deserialize, Serialize};

/// 笔记展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotesVO {
    pub id: Option<u64>,
    pub notebook_id: Option<u64>,
    pub label: Option<String>,
    pub topic: Option<String>,
    pub abstracts: Option<String>,
    pub path: Option<String>,
    pub content: Option<String>,
    pub source: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

impl NotesVO {
    pub fn from(arg: Notes,content: String) -> Self {
        Self {
            id: arg.id,
            notebook_id: arg.notebook_id,
            topic: arg.topic,
            label: arg.label,
            abstracts: arg.abstracts,
            path: None,
            content: Some(content),
            source: arg.source,
            create_time: arg.create_time,
            update_time: arg.update_time,
        }
    }
}
