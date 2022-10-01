use serde::{Deserialize, Serialize};
/// 任务归档数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanArchiveDTO {
    pub id:Option<u64>,
    pub plan_id:Option<u64>,
    pub content:Option<String>,
    pub status:Option<u32>,
    pub archive_time:Option<chrono::NaiveDateTime>,
    pub create_time: Option<chrono::NaiveDateTime>,
    pub update_time: Option<chrono::NaiveDateTime>,
}

/// 任务归档分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanArchivePageDTO{
    pub id:Option<u64>,
    pub plan_id:Option<u64>,
    pub content:Option<String>,
    pub status:Option<u32>,
    pub archive_time:Option<chrono::NaiveDateTime>,
    pub user:Option<String>,
    pub display:Option<u32>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub organize: Option<u64>
}