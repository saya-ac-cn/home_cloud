use serde::{Deserialize, Serialize};

/// 登录
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    // 账号
    pub account: Option<String>,
    // 密码
    pub password: Option<String>,
    // 平台
    pub platform: Option<String>
}
