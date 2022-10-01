use serde::{Deserialize, Serialize};
/// 通用笔记数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotesDTO {
    pub id:Option<u64>,
    pub notebook_id:Option<u64>,
    pub label:Option<String>,
    pub topic:Option<String>,
    pub content:Option<String>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}

/// 笔记分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotesPageDTO{
    pub id:Option<u64>,
    pub notebook_id:Option<u64>,
    pub label:Option<String>,
    pub topic:Option<String>,
    pub content:Option<String>,
    pub source:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub organize: Option<u64>
}