use serde::{Deserialize, Serialize};
/// 任务归档数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanArchiveDTO {
    /// 主键id
    pub id: Option<u64>,
    /// 标题
    pub title: Option<String>,
    /// 正文
    pub content: Option<String>,
    /// 被提醒者
    pub notice_user: Option<String>,
    /// 状态
    pub status: Option<u32>,
    /// 归属执行时间
    pub archive_time: Option<String>,
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

/// 任务归档分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlanArchivePageDTO {
    pub id: Option<u64>,
    pub content: Option<String>,
    pub status: Option<u32>,
    pub archive_time: Option<String>,
    pub user: Option<String>,
    pub display: Option<u32>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>,
}
