use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{ Error};
use crate::entity::domain::financial_database_tables::Journal;
use crate::entity::dto::journal::JournalDTO;

pub struct JournalMapper{}

impl JournalMapper {
    /// 修改流水
    #[html_sql("./src/dao/journal_mapper.html")]
    pub async fn update_journal(rb: &mut RbatisExecutor<'_, '_>, journal: &Journal) -> rbatis::core::Result<DBExecResult> { impled!() }
}