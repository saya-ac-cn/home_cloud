use serde::{Deserialize, Serialize};

/// 登陆
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    pub account: Option<String>,
    pub password: Option<String>,
}
