use crate::domain::domain::{User};
use serde::{Deserialize, Serialize};

///资源
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceVO {
    pub account: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
    pub login_check: Option<LoginCheck>,
    pub state: Option<i32>,
    pub del: Option<i32>,
    pub create_time: Option<rbatis::DateTimeNative>,
}
impl_field_name_method!(ResourceVO{account,password,name,login_check,state,del,create_time});
