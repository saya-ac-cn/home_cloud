use serde::{Deserialize, Serialize};
use crate::entity::domain::primary_database_tables::User;
use crate::util;
use crate::util::date_time::DateTimeUtil;

/// 用户展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserVO {
    /// 用户名
    pub account: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 性别
    pub sex: Option<String>,
    /// qq号
    pub qq: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 电话号码
    pub phone: Option<String>,
    /// 生日
    pub birthday: Option<String>,
    /// 故乡
    pub hometown: Option<String>,
    /// 签名
    pub autograph: Option<String>,
    /// 头像地址
    pub logo: Option<String>,
    /// 设置的背景
    pub background: Option<u64>,
    /// 所属组织
    pub organize_id: Option<u64>,
    /// 是否锁定(1正常，2锁定)
    pub state: Option<u32>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
    /// 壁纸url
    pub background_url: Option<String>
}
impl_field_name_method!(UserVO{account,name,password,sex,qq,email,phone,birthday,hometown,autograph,logo,background,organize_id,state,create_time,update_time});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserOwnOrganizeVO {
    /// 用户名
    pub account: Option<String>,
    /// 姓名
    pub name: Option<String>
}


impl From<User> for UserVO {
    fn from(arg: User) -> Self {
        Self {
            account: arg.account,
            name: arg.name,
            //屏蔽密码
            password: None,
            sex: arg.sex,
            qq: arg.qq,
            email: arg.email,
            phone: arg.phone,
            birthday: arg.birthday,
            hometown: arg.hometown,
            autograph: arg.autograph,
            logo: arg.logo,
            background: arg.background,
            organize_id: arg.organize_id,
            state: arg.state,
            background_url:None,
            create_time: DateTimeUtil::naive_date_time_to_str(&arg.create_time,&util::FORMAT_Y_M_D_H_M_S),
            update_time: DateTimeUtil::naive_date_time_to_str(&arg.update_time,&util::FORMAT_Y_M_D_H_M_S),
        }
    }
}