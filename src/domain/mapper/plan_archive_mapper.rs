use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{Error};
use crate::domain::table::PlanArchive;
use crate::domain::dto::plan_archive::PlanArchivePageDTO;
use crate::domain::dto::page::ExtendPageDTO;
use crate::domain::vo::plan_archive::PlanArchiveVO;

crud!(PlanArchive {});
impl_select!(PlanArchive{select_by_id(id:&u64) => "`where id = #{id}`"});
impl_delete!(PlanArchive{delete_by_id(id:&u64) => "`where id = #{id}`"});
pub struct PlanArchiveMapper{}

impl PlanArchiveMapper {

    /// 修改提醒事项
    #[html_sql("./src/domain/mapper/plan_archive_mapper.html")]
    pub async fn update_plan(rb: &mut dyn Executor,plan:&PlanArchive) -> rbatis::Result<ExecResult> { impled!() }

    /// 分页查询提醒事项
    #[html_sql("./src/domain/mapper/plan_archive_mapper.html")]
    pub async fn select_page(rb: &mut dyn Executor,plan:&PlanArchivePageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<PlanArchiveVO>>,Error> { impled!() }

    /// 查询提醒事项总数
    #[html_sql("./src/domain/mapper/plan_archive_mapper.html")]
    pub async fn select_count(rb: &mut dyn Executor,plan:&PlanArchivePageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }

    /// 查询未完成的提醒事项
    #[html_sql("./src/domain/mapper/plan_archive_mapper.html")]
    pub async fn select_undone_list(rb: &mut dyn Executor) -> Result<Option<Vec<PlanArchiveVO>>,Error> { impled!() }
}