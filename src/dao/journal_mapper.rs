use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{ Error};
use crate::entity::domain::financial_database_tables::Journal;
use crate::entity::dto::journal::JournalPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::journal::JournalVO;

pub struct JournalMapper{}

impl JournalMapper {
    /// 修改流水
    #[html_sql("./src/dao/journal_mapper.html")]
    pub async fn update_journal(rb: &mut RbatisExecutor<'_, '_>, journal: &Journal) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 分页查询流水
    #[html_sql("./src/dao/journal_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<JournalVO>>,Error> { impled!() }

    /// 查询流水总数
    #[html_sql("./src/dao/journal_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }

    /// 分页查询流水汇总
    #[html_sql("./src/dao/journal_mapper.html")]
    pub async fn select_day_page(rb: &mut RbatisExecutor<'_,'_>,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<JournalVO>>,Error> { impled!() }

    /// 查询流水汇总总数
    #[html_sql("./src/dao/journal_mapper.html")]
    pub async fn select_day_count(rb: &mut RbatisExecutor<'_,'_>,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }

    /// 查询指定月份的收支情况
    #[html_sql("./src/dao/journal_mapper.html")]
    pub async fn total_balance(rb: &mut RbatisExecutor<'_,'_>,organize:&u64,archive_date:&chrono::NaiveDate) -> Result<Option<Journal>,Error> { impled!() }

}