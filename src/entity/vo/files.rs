use serde::{Deserialize, Serialize};
use crate::entity::domain::business_database_tables::{Files};

/// 文件展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilesVO{
    pub id:Option<u64>,
    pub uid:Option<String>,
    pub file_name:Option<String>,
    pub file_url:Option<String>,
    pub file_type:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub status:Option<u32>,
    pub create_time:Option<String>,
    pub update_time:Option<String>
}

impl From<Files> for FilesVO {
    fn from(arg: Files) -> Self {
        Self {
            id:arg.id,
            uid:arg.uid,
            file_name:arg.file_name,
            file_url:arg.file_url,
            file_type:arg.file_type,
            organize: arg.organize,
            source:arg.source,
            status:arg.status,
            create_time: arg.create_time,
            update_time: arg.update_time
        }
    }
}
