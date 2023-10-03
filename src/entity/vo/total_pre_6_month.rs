use serde::{Deserialize, Serialize};

/// 统计最近半年中，每个月的指标
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TotalPre6MonthVO {
    pub total_month: Option<String>,
    pub count: Option<i32>,
}
