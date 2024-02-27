/// 主数据库
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// 用户名
    pub account: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 微信openId
    pub open_id: Option<String>,
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
    /// 是否锁定(1正常，2锁定冻结)
    pub state: Option<u32>,
    /// 创建时间
    pub create_time: Option<String>,
    /// 修改时间
    pub update_time: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Log {
    pub id: Option<u64>,
    pub organize: Option<u64>,
    pub user: Option<String>,
    pub category: Option<String>,
    pub ip: Option<String>,
    pub city: Option<String>,
    pub date: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogType {
    pub category: Option<String>,
    pub detail: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan {
    pub id: Option<u64>,
    pub standard_time: Option<String>,
    pub cycle: Option<u32>,
    pub unit: Option<u32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub notice_user: Option<String>,
    pub next_exec_time: Option<String>,
    pub check_up: Option<u32>,
    pub organize: Option<u64>,
    pub user: Option<String>,
    pub display: Option<u32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanArchive {
    pub id: Option<u64>,
    pub status: Option<u32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub notice_user: Option<String>,
    pub archive_time: Option<String>,
    pub organize: Option<u64>,
    pub user: Option<String>,
    pub display: Option<u32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DbDumpLog {
    pub id: Option<u64>,
    pub url: Option<String>,
    pub archive_date: Option<String>,
    pub execute_data: Option<String>,
}
