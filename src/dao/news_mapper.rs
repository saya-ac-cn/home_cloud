use actix_http::header::HeaderValue;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{DateTimeNative, Error};
use crate::entity::domain::business_database_tables::News;
use crate::entity::dto::news::NewsPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::news::NewsVO;

pub struct NewsMapper{}

impl NewsMapper {

    /// 修改动态
    #[html_sql("./src/dao/news_mapper.html")]
    pub async fn update_news(rb: &mut RbatisExecutor<'_,'_>,news:&News) -> rbatis::core::Result<DBExecResult> { impled!() }

    /// 分页查询动态
    #[html_sql("./src/dao/news_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,news:&NewsPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<NewsVO>>,Error> { impled!() }

    /// 查询动态总数
    #[html_sql("./src/dao/news_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,news:&NewsPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}