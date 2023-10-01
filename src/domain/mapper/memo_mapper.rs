use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{Error};
use crate::domain::table::Memo;
use crate::domain::dto::memo::MemoPageDTO;
use crate::domain::dto::page::ExtendPageDTO;
use crate::domain::vo::memo::MemoVO;

crud!(Memo {});
impl_select!(Memo {select_by_id_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});
impl_delete!(Memo {delete_by_id_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});


pub struct MemoMapper{}

impl MemoMapper {

    /// 修改便笺
    #[html_sql("./src/domain/mapper/memo_mapper.html")]
    pub async fn update_memo(rb: &mut dyn Executor,memo:&Memo) -> rbatis::Result<ExecResult> { impled!() }

    /// 分页查询便笺
    #[html_sql("./src/domain/mapper/memo_mapper.html")]
    pub async fn select_page(rb: &mut dyn Executor,memo:&MemoPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<MemoVO>>,Error> { impled!() }

    /// 查询便笺总数
    #[html_sql("./src/domain/mapper/memo_mapper.html")]
    pub async fn select_count(rb: &mut dyn Executor,memo:&MemoPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}