use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Base64PictureDTO {
    /// 文件名
    pub name: Option<String>,
    /// base64
    pub content: Option<String>,
    /// 会话token
    pub token: Option<String>,
}
