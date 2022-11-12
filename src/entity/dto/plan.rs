use serde::{Deserialize, Serialize};
/// 提醒事项数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanDTO {
    pub id:Option<u64>,
    pub standard_time:Option<String>,
    pub cycle:Option<u32>,
    pub unit:Option<u32>,
    pub title:Option<String>,
    pub content:Option<String>,
    pub next_exec_time:Option<String>,
    pub organize:Option<u64>,
    pub user:Option<String>,
    pub display:Option<u32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

/// 提醒事项分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanPageDTO{
    pub id:Option<u64>,
    pub standard_time:Option<String>,
    pub cycle:Option<u32>,
    pub unit:Option<u32>,
    pub title:Option<String>,
    pub content:Option<String>,
    pub next_exec_time:Option<String>,
    pub user:Option<String>,
    pub display:Option<u32>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>
}
