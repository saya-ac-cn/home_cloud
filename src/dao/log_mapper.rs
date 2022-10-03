use actix_http::header::HeaderValue;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{Error};
use rbatis::value::DateTimeNow;
use crate::entity::domain::primary_database_tables::Log;
use crate::entity::dto::log::LogPageDTO;
use crate::entity::dto::page::{ExtendPageDTO};
use crate::entity::vo::jwt::JWTToken;
use crate::entity::vo::log::LogVO;

pub struct LogMapper{}

impl LogMapper {

    /// 记录日志
    pub async fn record_log_by_jwt(rbatis: &Rbatis,jwt:&JWTToken, category: String) -> Result<rbatis::Result<DBExecResult>, Error> {
        let log = Log{
            id:None,
            organize:Some(jwt.organize),
            user:Some(jwt.account.clone()),
            category:Some(category),
            ip:Some(jwt.ip.clone()),
            city:Some(jwt.city.clone()),
            date:Some(chrono::NaiveDateTime::now()),
        };
       // let aa = DateTimeNative::now().inner;
       //  let aa1 = DateTimeNative::now();
       //  chrono::NaiveDateTime::now().
        return Ok(rbatis.save(&log, &[]).await);
    }

    /// 记录日志
    pub async fn record_log_by_token(rbatis: &Rbatis,token:Option<&HeaderValue>, category: String) -> Result<rbatis::Result<DBExecResult>, Error> {
        let extract_result = &JWTToken::extract_token_by_header(token);
        return match extract_result {
            Ok(token) => {
                LogMapper::record_log_by_jwt(rbatis, token, category).await
            }
            Err(e) => {
                Err(Error::from(e.to_string()))
            }
        }
    }

    /// 分页查询日志
    #[html_sql("./src/dao/log_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,log:&LogPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<LogVO>>,Error> { impled!() }

    /// 查询日志总数
    #[html_sql("./src/dao/log_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,log:&LogPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }

    /// 分页查询日志
    #[html_sql("./src/dao/log_mapper.html")]
    pub async fn select_recently(rb: &mut RbatisExecutor<'_,'_>,user:&String) -> Result<Option<LogVO>,Error> { impled!() }
}