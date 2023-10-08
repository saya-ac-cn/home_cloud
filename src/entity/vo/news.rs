use crate::entity::table::News;
use serde::{Deserialize, Serialize};

/// 消息动态展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewsVO {
    pub id: Option<u64>,
    pub topic: Option<String>,
    pub label: Option<String>,
    pub abstracts: Option<String>,
    pub path: Option<String>,
    pub content: Option<String>,
    pub organize: Option<u64>,
    pub source: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

impl NewsVO {
    pub fn from(arg: News,content: String) -> Self {
        Self {
            id: arg.id,
            topic: arg.topic,
            label: arg.label,
            abstracts: arg.abstracts,
            path: None,
            content: Some(content),
            organize: arg.organize,
            source: arg.source,
            create_time: arg.create_time,
            update_time: arg.update_time,
        }
    }
}
