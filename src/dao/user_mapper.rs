use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::user::UserPageDTO;
use crate::entity::vo::user::UserOwnOrganizeVO;
use crate::entity::domain::primary_database_tables::User;

pub struct UserMapper{}

impl UserMapper {

    ///根据账户名查找
    pub async fn find_by_account(rbatis: &Rbatis,account: &str) -> Result<Option<User>, rbatis::Error> {
        let wrapper = rbatis.new_wrapper().eq(User::account(), account);
        return Ok(rbatis.fetch_by_wrapper(wrapper).await?);
    }

    /// 修改用户信息
    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn update_user(rb: &mut RbatisExecutor<'_,'_>,user:&User) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 分页查询用户
    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,user:&UserPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<User>>,rbatis::Error> { impled!() }

    /// 查询用户总数
    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,user:&UserPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,rbatis::Error> { impled!() }

    /// 查询自己所在组织的用户列表
    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn select_own_organize_user(rb: &mut RbatisExecutor<'_,'_>,account:&String)-> Result<Option<Vec<UserOwnOrganizeVO>>,rbatis::Error> { impled!() }

}
