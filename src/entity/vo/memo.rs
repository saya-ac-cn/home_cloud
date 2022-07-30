use serde::{Deserialize, Serialize};
use crate::entity::domain::business_database_tables::{Memo};
use crate::util;
use crate::util::date_time::DateTimeUtil;

/// 便笺展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoVO{
    pub id:Option<u64>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub title:Option<String>,
    pub content:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>
}

impl From<Memo> for MemoVO {
    fn from(arg: Memo) -> Self {
        Self {
            id: arg.id,
            organize: arg.organize,
            source: arg.source,
            title: arg.title,
            content: arg.content,
            create_time: DateTimeUtil::naive_date_time_to_str(&arg.create_time,&util::FORMAT_Y_M_D_H_M_S),
            update_time: DateTimeUtil::naive_date_time_to_str(&arg.update_time,&util::FORMAT_Y_M_D_H_M_S)
        }
    }
}