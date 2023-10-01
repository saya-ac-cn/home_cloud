use serde::{Deserialize, Serialize};
use crate::domain::table::Notes;

/// 笔记展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotesVO{
    pub id:Option<u64>,
    pub notebook_id:Option<u64>,
    pub label:Option<String>,
    pub topic:Option<String>,
    pub abstracts:Option<String>,
    pub content:Option<String>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>
}

impl From<Notes> for NotesVO {
    fn from(arg: Notes) -> Self {
        Self {
            id: arg.id,
            notebook_id: arg.notebook_id,
            topic: arg.topic,
            label: arg.label,
            abstracts: arg.abstracts,
            content: arg.content,
            source: arg.source,
            create_time: arg.create_time,
            update_time: arg.update_time
        }
    }
}
