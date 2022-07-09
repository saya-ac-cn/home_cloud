use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use crate::entity::domain::User;
use rbatis::executor::{RbatisRef, RBatisTxExecutor, ExecutorMut, RbatisExecutor};
use rbatis::db::DBExecResult;
use crate::entity::dto::{ExtendPageDTO, UserPageDTO};

pub struct UserMapper{}

impl UserMapper {

    ///根据账户名查找
    pub async fn find_by_account(rbatis: &Rbatis,account: &str) -> Result<Option<User>, rbatis::Error> {
        let wrapper = rbatis.new_wrapper().eq(User::account(), account);
        return Ok(rbatis.fetch_by_wrapper(wrapper).await?);
    }

    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn update_user(rb: &mut RbatisExecutor<'_,'_>,user:&User) -> rbatis::core::Result<DBExecResult> { impled!() }

    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,user:&UserPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<User>>,rbatis::Error> { impled!() }

    #[html_sql("./src/dao/user_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,user:&UserPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,rbatis::Error> { impled!() }
}
