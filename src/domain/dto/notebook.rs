use serde::{Deserialize, Serialize};
/// 通用笔记簿数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoteBookDTO {
    /// 主键id
    pub id:Option<u64>,
    /// 笔记簿名
    pub name:Option<String>,
    /// 所属组织
    pub organize:Option<u64>,
    /// 所属用户
    pub source:Option<String>,
    /// 对外显示状态
    pub status:Option<u32>,
    /// 会话token
    pub token: Option<String>,
}