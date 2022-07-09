use serde::{Deserialize, Serialize};

/// 文件资源通用数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceDTO {
    pub id: Option<u64>,//资源id,
    pub name: Option<String>,//资源名称,
    pub suffix: Option<String>,// '资源扩展',
    pub size: Option<u64>,//文件大小,
    pub del: Option<i32>,
    pub create_user: Option<String>,//拥有者,
    pub create_time: Option<rbatis::DateTimeNative>,//上传时间
}
