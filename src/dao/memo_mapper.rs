use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{ Error};
use crate::entity::domain::business_database_tables::Memo;
use crate::entity::dto::memo::MemoPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::memo::MemoVO;

pub struct MemoMapper{}

impl MemoMapper {

    /// 修改便笺
    #[html_sql("./src/dao/memo_mapper.html")]
    pub async fn update_memo(rb: &mut RbatisExecutor<'_,'_>,memo:&Memo) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 分页查询便笺
    #[html_sql("./src/dao/memo_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,memo:&MemoPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<MemoVO>>,Error> { impled!() }

    /// 查询便笺总数
    #[html_sql("./src/dao/memo_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,memo:&MemoPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}