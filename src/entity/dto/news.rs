use serde::{Deserialize, Serialize};
/// 通用消息动态数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewsDTO {
    pub id:Option<u64>,
    pub topic:Option<String>,
    pub label:Option<String>,
    pub content:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}

/// 消息动态分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewsPageDTO{
    pub id:Option<u64>,
    pub topic:Option<String>,
    pub label:Option<String>,
    pub content:Option<String>,
    pub source:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub organize: Option<u64>
}