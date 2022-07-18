use serde::{Deserialize, Serialize};
use crate::entity::domain::business_database_tables::News;
use crate::util;
use crate::util::date_time::DateTimeUtil;

/// 消息动态展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewsVO{
    pub id:Option<u64>,
    pub topic:Option<String>,
    pub label:Option<String>,
    pub content:Option<String>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>
}

impl From<News> for NewsVO {
    fn from(arg: News) -> Self {
        Self {
            id: arg.id,
            topic: arg.topic,
            label: arg.label,
            content: arg.content,
            source: arg.source,
            create_time: DateTimeUtil::naive_date_time_to_str(&arg.create_time,&util::FORMAT_Y_M_D_H_M_S),
            update_time: DateTimeUtil::naive_date_time_to_str(&arg.update_time,&util::FORMAT_Y_M_D_H_M_S),
        }
    }
}