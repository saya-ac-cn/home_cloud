use serde::{Deserialize, Serialize};

/// 流水明细数据展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneralJournalVO {
    pub id: Option<u64>,
    pub journal_id: Option<u64>,
    pub flag: Option<String>,
    pub amount: Option<String>,
    pub remarks: Option<String>,
}

/// 流水明细汇总数据展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneralJournalCollectVO {
    pub id: Option<u64>,
    pub journal_id: Option<u64>,
    pub flag: Option<String>,
    pub amount: Option<String>,
    pub remarks: Option<String>,
    pub archive_date: Option<String>,
}
