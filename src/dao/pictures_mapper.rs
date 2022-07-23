use actix_http::header::HeaderValue;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{DateTimeNative, Error};
use crate::entity::domain::business_database_tables::News;
use crate::entity::dto::news::NewsPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::dto::pictures::PicturesPageDTO;
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::news::NewsVO;
use crate::entity::vo::pictures::PicturesVO;

pub struct PicturesMapper{}

impl PicturesMapper {

    /// 分页查询图片
    #[html_sql("./src/dao/pictures_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,pictures:&PicturesPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<PicturesVO>>,Error> { impled!() }

    /// 查询图片总数
    #[html_sql("./src/dao/pictures_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,pictures:&PicturesPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}