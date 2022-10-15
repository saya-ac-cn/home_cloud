use serde::{Deserialize, Serialize};
/// 图片数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PicturesDTO {
    pub id:Option<u64>,
    pub category:Option<u32>,
    pub file_name:Option<String>,
    pub descript:Option<String>,
    pub file_url:Option<String>,
    pub web_url:Option<String>,
    pub organize:Option<u64>,
    pub source:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>,
}

/// 图片分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PicturesPageDTO{
    pub id:Option<u64>,
    pub category:Option<u32>,
    pub file_name:Option<String>,
    pub descript:Option<String>,
    pub file_url:Option<String>,
    pub web_url:Option<String>,
    pub source:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>,
}
