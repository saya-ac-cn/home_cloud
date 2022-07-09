use crate::entity::domain::LoginCheck;
use rbatis::DateTimeNative;
use crate::entity::dto::UserPageDTO;


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

impl From<UserPageDTO> for User {
    fn from(arg: UserPageDTO) -> Self {
        Self{
            account:arg.account,
            name: arg.name,
            password: None,
            sex: None,
            qq: None,
            email: arg.email,
            phone: arg.phone,
            birthday: None,
            hometown: None,
            autograph: None,
            logo: None,
            background: None,
            organize_id: arg.organize_id,
            state: arg.state,
            create_time: None,
            update_time: None
        }
    }
}

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


///文件资源表
#[crud_table(table_name:resource)]
#[derive(Clone, Debug)]
pub struct Resource {
    pub id: Option<u64>,//资源id,
    pub name: Option<String>,//资源名称,
    pub suffix: Option<String>,// '资源扩展',
    pub size: Option<u64>,//文件大小,
    pub del: Option<i32>,
    pub create_user: Option<String>,//拥有者,
    pub create_time: Option<DateTimeNative>,//上传时间
}
impl_field_name_method!(Resource{id,name,suffix,size,del,create_user,create_time});
