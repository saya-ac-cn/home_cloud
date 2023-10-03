use serde::{Deserialize, Serialize};
/// 提醒事项数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanDTO {
    /// 主键id
    pub id: Option<u64>,
    /// 基准时间
    pub standard_time: Option<String>,
    /// 周期频率
    pub cycle: Option<u32>,
    /// 单位
    pub unit: Option<u32>,
    /// 标题
    pub title: Option<String>,
    /// 正文
    pub content: Option<String>,
    /// 下次执行时间
    pub next_exec_time: Option<String>,
    /// 归属组织
    pub organize: Option<u64>,
    /// 归属用户
    pub user: Option<String>,
    /// 是否对外呈现
    pub display: Option<u32>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
    /// 会话token
    pub token: Option<String>,
}

/// 提醒事项分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanPageDTO {
    pub id: Option<u64>,
    pub standard_time: Option<String>,
    pub cycle: Option<u32>,
    pub unit: Option<u32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub next_exec_time: Option<String>,
    pub user: Option<String>,
    pub display: Option<u32>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>,
}
