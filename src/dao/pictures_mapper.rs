use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::{Error};
use crate::entity::domain::business_database_tables::Pictures;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::pictures::PicturesPageDTO;
use crate::entity::vo::pictures::PicturesVO;

crud!(Pictures {});
pub struct PicturesMapper{}

impl PicturesMapper {

    /// 分页查询图片
    #[html_sql("./src/dao/pictures_mapper.html")]
    pub async fn select_page(rb: &mut dyn Executor,pictures:&PicturesPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<PicturesVO>>,Error> { impled!() }

    /// 查询图片总数
    #[html_sql("./src/dao/pictures_mapper.html")]
    pub async fn select_count(rb: &mut dyn Executor,pictures:&PicturesPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}