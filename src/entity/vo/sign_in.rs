use serde::{Deserialize, Serialize};
use crate::entity::vo::log::LogVO;
use crate::entity::vo::user::UserVO;

///登录成功后的凭证数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignInVO {
    pub user: Option<UserVO>,
    pub access_token: String,
    pub plan:Option<rbson::Array>,
    pub log:Option<LogVO>
}

impl ToString for SignInVO {
    fn to_string(&self) -> String {
        serde_json::json!(self).to_string()
    }
}
