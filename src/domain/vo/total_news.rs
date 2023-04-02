use serde::{Deserialize, Serialize};
use crate::domain::vo::total_pre_6_month::TotalPre6MonthVO;

/// 动态统计响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TotalNewsVO {
    pub avg: Option<u64>,
    pub count: Option<u64>,
    pub news6:Option<Vec<TotalPre6MonthVO>>
}