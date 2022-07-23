use serde::{Deserialize, Serialize};
/// 文件数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesDTO {
    pub id:Option<u64>,
    pub uid:Option<String>,
    pub file_name:Option<String>,
    pub file_url:Option<String>,
    pub file_type:Option<String>,
    pub source:Option<String>,
    pub status:Option<String>,
    pub date:Option<String>
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
    pub status:Option<String>,
    pub date:Option<String>,

    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<rbatis::DateTimeNative>,
    pub end_time: Option<rbatis::DateTimeNative>,
    pub organize: Option<Vec<String>>
}