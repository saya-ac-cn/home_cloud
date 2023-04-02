use serde::{Deserialize, Serialize};
use crate::domain::dto::general_journal::GeneralJournalDTO;

/// 流水数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalDTO {
    pub id:Option<u64>,
    pub monetary_id:Option<u64>,
    pub means_id:Option<u64>,
    pub abstract_id:Option<u64>,
    pub remarks:Option<String>,
    pub archive_date:Option<String>,
    pub details:Option<Vec<GeneralJournalDTO>>
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
