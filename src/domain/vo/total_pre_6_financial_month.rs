use serde::{Deserialize, Serialize};

/// 统计最近半年中，每个月的财务指标
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TotalPre6MonthFinancialVO {
    pub archive_date:Option<String>,
    pub income:Option<String>,
    pub outlay:Option<String>,
    pub total:Option<String>
}