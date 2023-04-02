use serde::{Deserialize, Serialize};
/// 便笺数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemoDTO {
    pub id:Option<u64>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub title:Option<String>,
    pub content:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}

/// 便笺分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemoPageDTO{
    pub id:Option<u64>,
    pub source:Option<String>,
    pub title:Option<String>,
    pub content:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>
}
