use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{ Error};
use crate::entity::domain::primary_database_tables::Plan;
use crate::entity::dto::plan::PlanPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::plan::PlanVO;

pub struct PlanMapper{}

impl PlanMapper {

    /// 修改提醒事项
    #[html_sql("./src/dao/plan_mapper.html")]
    pub async fn update_plan(rb: &mut RbatisExecutor<'_,'_>,plan:&Plan) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 分页查询提醒事项
    #[html_sql("./src/dao/plan_mapper.html")]
    pub async fn select_list(rb: &mut RbatisExecutor<'_,'_>,plan:&PlanPageDTO) -> Result<Option<Vec<Plan>>,Error> { impled!() }

    /// 分页查询提醒事项
    #[html_sql("./src/dao/plan_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,plan:&PlanPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<PlanVO>>,Error> { impled!() }

    /// 查询提醒事项总数
    #[html_sql("./src/dao/plan_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,plan:&PlanPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}