use serde::{Deserialize, Serialize};
/// 文件数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesDTO {
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


/// 文件分页数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesPageDTO{
    pub id:Option<u64>,
    pub uid:Option<String>,
    pub file_name:Option<String>,
    pub file_url:Option<String>,
    pub file_type:Option<String>,
    pub source:Option<String>,
    pub status:Option<u32>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
    pub organize: Option<u64>
}