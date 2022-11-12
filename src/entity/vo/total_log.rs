use serde::{Deserialize, Serialize};
use crate::entity::vo::total_pre_6_month::TotalPre6MonthVO;

/// 活跃度统计（日志）响应数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TotalLogVO {
    pub avg: Option<u64>,
    pub count: Option<u64>,
    pub log6:Option<Vec<TotalPre6MonthVO>>
}