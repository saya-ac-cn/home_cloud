use serde::{Deserialize, Serialize};
use crate::entity::domain::business_database_tables::Notes;
use crate::util;
use crate::util::date_time::DateTimeUtil;

/// 笔记展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NotesVO{
    pub id:Option<u64>,
    pub notebook_id:Option<u64>,
    pub label:Option<String>,
    pub topic:Option<String>,
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
            content: arg.content,
            source: arg.source,
            create_time: DateTimeUtil::naive_date_time_to_str(&arg.create_time,&util::FORMAT_Y_M_D_H_M_S),
            update_time: DateTimeUtil::naive_date_time_to_str(&arg.update_time,&util::FORMAT_Y_M_D_H_M_S)
        }
    }
}