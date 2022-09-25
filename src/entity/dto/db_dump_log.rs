use serde::{Deserialize, Serialize};
/// 通用数据库备份日志数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbDumpLogDTO {
    pub id:Option<u64>,
    pub url:Option<String>,
    pub archive_date:Option<rbatis::DateNative>,
    pub execute_data: Option<rbatis::DateTimeNative>,
}

/// 数据库备份日志数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbDumpLogPageDTO{
    pub id:Option<u64>,
    pub url:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<rbatis::DateTimeNative>,
    pub end_time: Option<rbatis::DateTimeNative>,
}