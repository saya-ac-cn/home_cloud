use serde::{Deserialize, Serialize};
/// 任务归档数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanArchiveDTO {
    pub id:Option<u64>,
    pub title:Option<String>,
    pub content:Option<String>,
    pub status:Option<u32>,
    pub archive_time:Option<String>,
    pub organize:Option<u64>,
    pub user:Option<String>,
    pub display:Option<u32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

/// 任务归档分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanArchivePageDTO{
    pub id:Option<u64>,
    pub content:Option<String>,
    pub status:Option<u32>,
    pub archive_time:Option<String>,
    pub user:Option<String>,
    pub display:Option<u32>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>
}
