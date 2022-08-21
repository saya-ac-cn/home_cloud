use serde::{Deserialize, Serialize};
/// 提醒事项数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanDTO {
    pub id:Option<u64>,
    pub standard_time:Option<rbatis::DateTimeNative>,
    pub cycle:Option<u32>,
    pub unit:Option<u32>,
    pub content:Option<String>,
    pub last_exec_time:Option<rbatis::DateTimeNative>,
    pub organize:Option<u64>,
    pub user:Option<String>,
    pub display:Option<u32>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

/// 提醒事项分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanPageDTO{
    pub id:Option<u64>,
    pub standard_time:Option<rbatis::DateTimeNative>,
    pub cycle:Option<u32>,
    pub unit:Option<u32>,
    pub content:Option<String>,
    pub last_exec_time:Option<rbatis::DateTimeNative>,
    pub user:Option<String>,
    pub display:Option<u32>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<rbatis::DateTimeNative>,
    pub end_time: Option<rbatis::DateTimeNative>,
    pub organize: Option<u64>
}