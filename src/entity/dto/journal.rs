use rbatis::{DateNative};
use serde::{Deserialize, Serialize};
use crate::entity::dto::general_journal::GeneralJournalDTO;

/// 流水数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JournalDTO {
    pub id:Option<u64>,
    pub monetary_id:Option<u64>,
    pub means_id:Option<u64>,
    pub amount_id:Option<u64>,
    pub remarks:Option<String>,
    pub archive_date:Option<DateNative>,
    pub details:Option<Vec<GeneralJournalDTO>>
}