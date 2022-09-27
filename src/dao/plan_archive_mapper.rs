use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{ Error};
use crate::entity::domain::primary_database_tables::PlanArchive;
use crate::entity::dto::plan_archive::PlanArchivePageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::plan_archive::PlanArchiveVO;

pub struct PlanArchiveMapper{}

impl PlanArchiveMapper {

    /// 修改提醒事项
    #[html_sql("./src/dao/plan_archive_mapper.html")]
    pub async fn update_plan(rb: &mut RbatisExecutor<'_,'_>,plan:&PlanArchive) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 分页查询提醒事项
    #[html_sql("./src/dao/plan_archive_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,plan:&PlanArchivePageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<PlanArchiveVO>>,Error> { impled!() }

    /// 查询提醒事项总数
    #[html_sql("./src/dao/plan_archive_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,plan:&PlanArchivePageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }

    /// 查询未完成的提醒事项
    #[html_sql("./src/dao/plan_archive_mapper.html")]
    pub async fn select_undone_list(rb: &mut RbatisExecutor<'_,'_>) -> Result<Option<Vec<PlanArchiveVO>>,Error> { impled!() }
}