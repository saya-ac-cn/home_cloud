use serde::{Deserialize, Serialize};
/// 便笺数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemoDTO {
    /// 主键id
    pub id: Option<u64>,
    /// 所属组织
    pub organize: Option<u64>,
    /// 所属用户
    pub source: Option<String>,
    /// 标题
    pub title: Option<String>,
    /// 正文
    pub content: Option<String>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
    /// 会话token
    pub token: Option<String>,
}

/// 便笺分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MemoPageDTO {
    pub id: Option<u64>,
    pub source: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>,
}
