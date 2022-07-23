use serde::{Deserialize, Serialize};
use crate::entity::domain::business_database_tables::{Pictures};
use crate::util;
use crate::util::date_time::DateTimeUtil;

/// 图片展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PicturesVO{
    pub id:Option<u64>,
    pub category:Option<u32>,
    pub file_name:Option<String>,
    pub descript:Option<String>,
    pub file_url:Option<String>,
    pub web_url:Option<String>,
    pub source:Option<String>,
    pub date:Option<String>,
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
            source:arg.source,
            date: DateTimeUtil::naive_date_time_to_str(&arg.date,&util::FORMAT_Y_M_D_H_M_S),
        }
    }
}