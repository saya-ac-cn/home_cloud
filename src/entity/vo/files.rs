use serde::{Deserialize, Serialize};
use crate::entity::domain::business_database_tables::{Files};
use crate::util;
use crate::util::date_time::DateTimeUtil;

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
            create_time: DateTimeUtil::naive_date_time_to_str(&arg.create_time,&util::FORMAT_Y_M_D_H_M_S),
            update_time: DateTimeUtil::naive_date_time_to_str(&arg.update_time,&util::FORMAT_Y_M_D_H_M_S)
        }
    }
}