use serde::{Deserialize, Serialize};
/// 通用数据库备份日志数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbDumpLogDTO {
    /// 主键id
    pub id:Option<u64>,
    /// 本地存储位置
    pub url:Option<String>,
    /// 归档时间
    pub archive_date:Option<String>,
    /// 执行时间
    pub execute_data: Option<String>,
}

/// 数据库备份日志数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbDumpLogPageDTO{
    pub id:Option<u64>,
    pub url:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}
