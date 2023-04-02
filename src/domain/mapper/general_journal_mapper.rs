use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{Error};
use crate::domain::table::{GeneralJournal};
use crate::domain::dto::journal::JournalPageDTO;
use crate::domain::dto::page::ExtendPageDTO;
use crate::domain::vo::general_journal::{GeneralJournalCollectVO, GeneralJournalVO};

crud!(GeneralJournal {});

pub struct GeneralJournalMapper{}

impl GeneralJournalMapper {
    /// 修改流水明细
    #[html_sql("./src/domain/mapper/general_journal_mapper.html")]
    pub async fn update_general_journal(rb: &mut dyn Executor, general_journal: &GeneralJournal) -> rbatis::Result<ExecResult> { impled!() }

    /// 删除流水明细
    #[html_sql("./src/domain/mapper/general_journal_mapper.html")]
    pub async fn delete_general_journal(rb: &mut dyn Executor, id:&u64) -> rbatis::Result<ExecResult> { impled!() }

    /// 查询流水明细
    #[html_sql("./src/domain/mapper/general_journal_mapper.html")]
    pub async fn select_detail(rb: &mut dyn Executor,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<GeneralJournalVO>>,Error> { impled!() }

    /// 查询按天汇总的流水明细
    #[html_sql("./src/domain/mapper/general_journal_mapper.html")]
    pub async fn select_day_collect(rb: &mut dyn Executor,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<GeneralJournalCollectVO>>,Error> { impled!() }

    /// 查询流水明细总数
    #[html_sql("./src/domain/mapper/general_journal_mapper.html")]
    pub async fn select_count_by_journal_id(rb: &mut dyn Executor,journal_id:&u64) -> Result<Option<u64>,Error> { impled!() }
}