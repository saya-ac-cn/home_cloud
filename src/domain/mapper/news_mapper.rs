use rbatis::executor::Executor;
use rbatis::{Error};
use rbatis::rbdc::db::ExecResult;
use crate::domain::table::News;
use crate::domain::dto::news::NewsPageDTO;
use crate::domain::dto::page::ExtendPageDTO;
use crate::domain::vo::news::NewsVO;
crud!(News {});

impl_select!(News {select_by_id_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});
impl_select!(News {select_by_ids(ids:Vec<u64>) => "`where id in (#{id})`"});
impl_delete!(News {delete_by_id_organize(id:&u64,organize:&u64) => "`where id = #{id} and organize= #{organize}`"});

pub struct NewsMapper{}


impl NewsMapper {

    /// 修改动态
    #[html_sql("./src/domain/mapper/news_mapper.html")]
    pub async fn update_news(rb: &mut dyn Executor,news:&News) -> rbatis::Result<ExecResult> { impled!() }

    /// 分页查询动态
    #[html_sql("./src/domain/mapper/news_mapper.html")]
    pub async fn select_page(rb: &mut dyn Executor,news:&NewsPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<NewsVO>>,Error> { impled!() }

    /// 查询动态总数
    #[html_sql("./src/domain/mapper/news_mapper.html")]
    pub async fn select_count(rb: &mut dyn Executor,news:&NewsPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}
