use serde::{Deserialize, Serialize};
/// 图片数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PicturesDTO {
    /// 主键id
    pub id:Option<u64>,
    /// 所属分类
    pub category:Option<u32>,
    /// 文件名
    pub file_name:Option<String>,
    /// 描述
    pub descript:Option<String>,
    /// 服务器上存储路径
    pub file_url:Option<String>,
    /// 对外访问相对路径
    pub web_url:Option<String>,
    /// 所属组织
    pub organize:Option<u64>,
    /// 所属用户
    pub source:Option<String>,
    /// 创建时间
    pub create_time:Option<String>,
    /// 修改时间
    pub update_time:Option<String>,
    /// 会话token
    pub token: Option<String>,
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
