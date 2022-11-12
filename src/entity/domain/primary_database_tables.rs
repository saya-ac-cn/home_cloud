/// 主数据库
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
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
}
crud!(User {});
impl_select!(User{select_by_account(account:&String) => "`where account = #{account}`"});
impl_field_name_method!(User{account,name,email,phone,organize_id,state});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Log{
    pub id:Option<u64>,
    pub organize:Option<u64>,
    pub user:Option<String>,
    pub category:Option<String>,
    pub ip:Option<String>,
    pub city:Option<String>,
    pub date:Option<String>,
}
crud!(Log {});
impl_field_name_method!(Log{id,user,category,date});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogType{
    pub category:Option<String>,
    pub detail:Option<String>,
}
crud!(LogType {});
impl_field_name_method!(LogType{category});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Plan{
    pub id:Option<u64>,
    pub standard_time:Option<String>,
    pub cycle:Option<u32>,
    pub unit:Option<u32>,
    pub title:Option<String>,
    pub content:Option<String>,
    pub next_exec_time:Option<String>,
    pub organize:Option<u64>,
    pub user:Option<String>,
    pub display:Option<u32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
crud!(Plan {});
impl_delete!(Plan {delete_by_id_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});
impl_field_name_method!(Plan{id,organize,user,display,standard_time,next_exec_time});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanArchive{
    pub id:Option<u64>,
    pub status:Option<u32>,
    pub title:Option<String>,
    pub content:Option<String>,
    pub archive_time:Option<String>,
    pub organize:Option<u64>,
    pub user:Option<String>,
    pub display:Option<u32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}
crud!(PlanArchive {});
impl_field_name_method!(PlanArchive{id,status,archive_time});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DbDumpLog{
    pub id:Option<u64>,
    pub url:Option<String>,
    pub archive_date:Option<String>,
    pub execute_data:Option<String>,
}
crud!(DbDumpLog {});
impl_field_name_method!(DbDumpLog{id,archive_date});
