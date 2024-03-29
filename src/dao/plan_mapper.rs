use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::plan::PlanPageDTO;
use crate::entity::table::Plan;
use crate::entity::vo::plan::PlanVO;
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::Error;

crud!(Plan {});
impl_delete!(Plan {delete_by_id_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});
impl_select!(Plan{select_by_id(id:&u64) => "`where id = #{id}`"});
impl_select!(Plan{select_current_after() => "`where standard_time >= now()`"});

pub struct PlanMapper {}

impl PlanMapper {
    /// 修改提醒事项
    #[html_sql("./src/dao/plan_mapper.html")]
    pub async fn update_plan(rb: &mut dyn Executor, plan: &Plan) -> rbatis::Result<ExecResult> {
        impled!()
    }

    /// 分页查询提醒事项
    #[html_sql("./src/dao/plan_mapper.html")]
    pub async fn select_list(
        rb: &mut dyn Executor,
        plan: &PlanPageDTO,
    ) -> Result<Option<Vec<Plan>>, Error> {
        impled!()
    }

    /// 分页查询提醒事项
    #[html_sql("./src/dao/plan_mapper.html")]
    pub async fn select_page(
        rb: &mut dyn Executor,
        plan: &PlanPageDTO,
        extend: &ExtendPageDTO,
    ) -> Result<Option<Vec<PlanVO>>, Error> {
        impled!()
    }

    /// 查询提醒事项总数
    #[html_sql("./src/dao/plan_mapper.html")]
    pub async fn select_count(
        rb: &mut dyn Executor,
        plan: &PlanPageDTO,
        extend: &ExtendPageDTO,
    ) -> Result<Option<u64>, Error> {
        impled!()
    }
}
