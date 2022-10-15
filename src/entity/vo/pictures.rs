use serde::{Deserialize, Serialize};
use crate::entity::domain::business_database_tables::{Pictures};

/// 图片展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PicturesVO{
    pub id:Option<u64>,
    pub category:Option<u32>,
    pub file_name:Option<String>,
    pub descript:Option<String>,
    pub file_url:Option<String>,
    pub web_url:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>
}

impl From<Pictures> for PicturesVO {
    fn from(arg: Pictures) -> Self {
        Self {
            id:arg.id,
            category:arg.category,
            file_name:arg.file_name,
            descript:arg.descript,
            file_url:arg.file_url,
            web_url:arg.web_url,
            organize:arg.organize,
            source:arg.source,
            create_time: arg.create_time,
            update_time: arg.update_time
        }
    }
}
