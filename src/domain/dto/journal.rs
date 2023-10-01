use serde::{Deserialize, Serialize};
use crate::domain::dto::general_journal::GeneralJournalDTO;

/// 流水数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalDTO {
    /// 主键id
    pub id:Option<u64>,
    /// 货币类型
    pub monetary_id:Option<u64>,
    /// 收支方式
    pub means_id:Option<u64>,
    /// 摘要
    pub abstract_id:Option<u64>,
    /// 备注
    pub remarks:Option<String>,
    /// 归档时间
    pub archive_date:Option<String>,
    /// 流水详情
    pub details:Option<Vec<GeneralJournalDTO>>,
    /// 会话token
    pub token: Option<String>,
}

/// 流水数据分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalPageDTO{
    pub id:Option<u64>,
    pub monetary_id:Option<u64>,
    pub means_id:Option<u64>,
    pub abstract_id:Option<u64>,
    pub source:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>
}

/// 流水数据统计传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalTotalDTO{
    pub archive_date:String,
}
