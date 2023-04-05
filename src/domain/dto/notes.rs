use serde::{Deserialize, Serialize};
/// 通用笔记数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotesDTO {
    /// 主键id
    pub id:Option<u64>,
    /// 归属笔记簿
    pub notebook_id:Option<u64>,
    /// 标签
    pub label:Option<String>,
    /// 主题
    pub topic:Option<String>,
    /// 简述
    pub abstracts:Option<String>,
    /// 正文
    pub content:Option<String>,
    /// 所属用户
    pub source:Option<String>,
    /// 创建时间
    pub create_time:Option<String>,
    /// 修改时间
    pub update_time:Option<String>,
    /// 会话token
    pub token: Option<String>,
}

/// 笔记分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NotesPageDTO{
    pub id:Option<u64>,
    pub notebook_id:Option<u64>,
    pub label:Option<String>,
    pub topic:Option<String>,
    pub content:Option<String>,
    pub status:Option<u32>,
    pub source:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>
}
