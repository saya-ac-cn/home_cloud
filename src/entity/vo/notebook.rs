use serde::{Deserialize, Serialize};
use crate::entity::domain::business_database_tables::NoteBook;

/// 笔记簿展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteBookVO{
    pub id:Option<u64>,
    pub name:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub status:Option<u32>,
    pub descript:Option<String>,
    pub notes_count:Option<u64>
}

impl From<NoteBook> for NoteBookVO {
    fn from(arg: NoteBook) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            organize: arg.organize,
            source: arg.source,
            status: arg.status,
            descript: arg.descript,
            notes_count:None
        }
    }
}