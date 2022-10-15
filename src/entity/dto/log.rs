use serde::{Deserialize, Serialize};
/// 通用日志数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogDTO {
    pub id:Option<u64>,
    pub organize: Option<u64>,
    pub user:Option<String>,
    pub category:Option<String>,
    pub ip:Option<String>,
    pub city:Option<String>,
    pub date:Option<String>,
}

/// 日志分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogPageDTO{
    pub id:Option<u64>,
    pub organize: Option<u64>,
    pub user:Option<String>,
    pub category:Option<String>,
    pub ip:Option<String>,
    pub city:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}
