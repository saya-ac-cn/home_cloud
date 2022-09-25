use rbatis::{DateNative, DateTimeNative};

/// 主数据库

#[crud_table(table_name:user)]
#[derive(Clone, Debug)]
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
    pub create_time: Option<DateTimeNative>,
    /// 修改时间
    pub update_time: Option<DateTimeNative>,
}
impl_field_name_method!(User{account,name,password,sex,qq,email,phone,birthday,hometown,autograph,logo,background,organize_id,state,create_time,update_time});

#[crud_table(table_name:log)]
#[derive(Clone, Debug)]
pub struct Log{
    pub id:Option<u64>,
    pub organize:Option<u64>,
    pub user:Option<String>,
    pub category:Option<String>,
    pub ip:Option<String>,
    pub city:Option<String>,
    pub date:Option<DateTimeNative>,
}
impl_field_name_method!(Log{id,user,category,ip,city,date});

#[crud_table(table_name:log_type)]
#[derive(Clone, Debug)]
pub struct LogType{
    pub category:Option<String>,
    pub describe:Option<String>,
}
impl_field_name_method!(LogType{category,describe});

#[crud_table(table_name:plan)]
#[derive(Clone, Debug)]
pub struct Plan{
    pub id:Option<u64>,
    pub standard_time:Option<DateTimeNative>,
    pub cycle:Option<u32>,
    pub unit:Option<u32>,
    pub content:Option<String>,
    pub next_exec_time:Option<DateTimeNative>,
    pub organize:Option<u64>,
    pub user:Option<String>,
    pub display:Option<u32>,
    pub create_time: Option<DateTimeNative>,
    pub update_time: Option<DateTimeNative>,
}
impl_field_name_method!(Plan{id,organize,user,display,standard_time,next_exec_time});

#[crud_table(table_name:plan_archive)]
#[derive(Clone, Debug)]
pub struct PlanArchive{
    pub id:Option<u64>,
    pub plan_id:Option<u64>,
    pub status:Option<u32>,
    pub content:Option<String>,
    pub archive_time:Option<DateTimeNative>,
    pub create_time: Option<DateTimeNative>,
    pub update_time: Option<DateTimeNative>,
}
impl_field_name_method!(PlanArchive{id,plan_id,status,archive_time});

#[crud_table(table_name:db_dump_log)]
#[derive(Clone, Debug)]
pub struct DbDumpLog{
    pub id:Option<u64>,
    pub url:Option<String>,
    pub archive_date:Option<DateNative>,
    pub execute_data:Option<DateTimeNative>,
}
impl_field_name_method!(DbDumpLog{id,archive_date});