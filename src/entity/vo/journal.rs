use serde::{Deserialize, Serialize};

/// 流水数据展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JournalVO {
    pub id: Option<u64>,
    pub monetary_id: Option<u64>,
    pub income: Option<String>,
    pub outlay: Option<String>,
    pub means_id: Option<u64>,
    pub abstract_id: Option<u64>,
    pub total: Option<String>,
    pub remarks: Option<String>,
    pub archive_date: Option<String>,
    pub organize: Option<u64>,
    pub source: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub payment_means_name: Option<String>,
    pub abstracts_name: Option<String>,
    pub monetary_name: Option<String>,
}
