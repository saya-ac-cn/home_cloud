use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{ Error};
use crate::entity::domain::financial_database_tables::{GeneralJournal};
use crate::entity::dto::journal::JournalPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::general_journal::GeneralJournalVO;


pub struct GeneralJournalMapper{}

impl GeneralJournalMapper {
    /// 修改流水明细
    #[html_sql("./src/dao/general_journal_mapper.html")]
    pub async fn update_general_journal(rb: &mut RbatisExecutor<'_, '_>, general_journal: &GeneralJournal) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 删除流水明细
    #[html_sql("./src/dao/general_journal_mapper.html")]
    pub async fn delete_general_journal(rb: &mut RbatisExecutor<'_, '_>, id:&u64) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 分页查询流水
    #[html_sql("./src/dao/general_journal_mapper.html")]
    pub async fn select_detail(rb: &mut RbatisExecutor<'_,'_>,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<GeneralJournalVO>>,Error> { impled!() }

}