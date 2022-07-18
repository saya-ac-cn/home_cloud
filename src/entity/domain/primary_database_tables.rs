use rbatis::DateTimeNative;

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