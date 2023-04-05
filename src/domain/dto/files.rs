use serde::{Deserialize, Serialize};
/// 文件数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FilesDTO {
    /// 主键id
    pub id:Option<u64>,
    /// 前端生成的随机id
    pub uid:Option<String>,
    /// 文件名
    pub file_name:Option<String>,
    /// 本地存储位置
    pub file_url:Option<String>,
    /// 文件类型
    pub file_type:Option<String>,
    /// 所属组织
    pub organize:Option<u64>,
    /// 所属用户
    pub source:Option<String>,
    /// 对完显示状态
    pub status:Option<u32>,
    /// 创建时间
    pub create_time:Option<String>,
    /// 修改时间
    pub update_time:Option<String>,
    /// 会话token
    pub token: Option<String>,
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
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
    pub organize: Option<u64>
}
