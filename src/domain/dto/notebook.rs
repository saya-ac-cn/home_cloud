use serde::{Deserialize, Serialize};
/// 通用笔记簿数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NoteBookDTO {
    pub id:Option<u64>,
    pub name:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub status:Option<u32>,
    pub descript:Option<String>
}