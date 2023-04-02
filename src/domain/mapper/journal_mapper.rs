use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{Error};
use crate::domain::table::Journal;
use crate::domain::dto::journal::JournalPageDTO;
use crate::domain::dto::page::ExtendPageDTO;
use crate::domain::vo::journal::JournalVO;

crud!(Journal {});
impl_select!(Journal {select_by_id_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});
impl_delete!(Journal {delete_by_id_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});


pub struct JournalMapper{}

impl JournalMapper {
    /// 修改流水
    #[html_sql("./src/domain/mapper/journal_mapper.html")]
    pub async fn update_journal(rb: &mut dyn Executor, journal: &Journal) -> rbatis::Result<ExecResult> { impled!() }

    /// 分页查询流水
    #[html_sql("./src/domain/mapper/journal_mapper.html")]
    pub async fn select_page(rb: &mut dyn Executor,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<JournalVO>>,Error> { impled!() }

    /// 查询流水总数
    #[html_sql("./src/domain/mapper/journal_mapper.html")]
    pub async fn select_count(rb: &mut dyn Executor,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }

    /// 分页查询流水汇总
    #[html_sql("./src/domain/mapper/journal_mapper.html")]
    pub async fn select_day_page(rb: &mut dyn Executor,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<JournalVO>>,Error> { impled!() }

    /// 查询流水汇总总数
    #[html_sql("./src/domain/mapper/journal_mapper.html")]
    pub async fn select_day_count(rb: &mut dyn Executor,journal:&JournalPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }

    /// 查询指定月份的收支情况
    #[html_sql("./src/domain/mapper/journal_mapper.html")]
    pub async fn total_balance(rb: &mut dyn Executor,organize:&u64,archive_date:&chrono::NaiveDate) -> Result<Option<Journal>,Error> { impled!() }

    /// 月度账单排序
    #[html_sql("./src/domain/mapper/journal_mapper.html")]
    pub async fn bill_rank(rb: &mut dyn Executor,organize:&u64,archive_date:&chrono::NaiveDate) -> Result<Option<Vec<JournalVO>>,Error> { impled!() }

}