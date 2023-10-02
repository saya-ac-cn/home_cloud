use actix_http::header::HeaderValue;

use crate::entity::dto::log::LogPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::table::Log;
use crate::entity::vo::log::LogVO;
use crate::util::date_time::{DateTimeUtil, DateUtils};
use rbatis::executor::Executor;
use rbatis::rbdc::db::ExecResult;
use rbatis::Error;

use crate::entity::vo::user_context::UserContext;
use crate::util;
crud!(Log {});

pub struct LogMapper {}

impl LogMapper {
    /// 记录日志
    pub async fn record_log_by_context(
        rb: &mut dyn Executor,
        context: &UserContext,
        category: String,
    ) -> Result<rbatis::Result<ExecResult>, Error> {
        let log = Log {
            id: None,
            organize: Some(context.organize),
            user: Some(context.account.clone()),
            category: Some(category),
            ip: Some(context.ip.clone()),
            city: Some(context.city.clone()),
            date: DateTimeUtil::naive_date_time_to_str(
                &Some(DateUtils::now()),
                &util::FORMAT_Y_M_D_H_M_S,
            ),
        };
        return Ok(Log::insert(rb, &log).await);
    }

    /// 记录日志
    pub async fn record_log_by_token(
        rb: &mut dyn Executor,
        token: Option<&HeaderValue>,
        category: String,
    ) -> Result<rbatis::Result<ExecResult>, Error> {
        let extract_result = &UserContext::extract_token_by_header(token).await;
        return match extract_result {
            Ok(token) => LogMapper::record_log_by_context(rb, token, category).await,
            Err(e) => Err(Error::from(e.to_string())),
        };
    }

    /// 分页查询日志
    #[html_sql("./src/dao/log_mapper.html")]
    pub async fn select_page(
        rb: &mut dyn Executor,
        log: &LogPageDTO,
        extend: &ExtendPageDTO,
    ) -> Result<Option<Vec<LogVO>>, Error> {
        impled!()
    }

    /// 查询日志总数
    #[html_sql("./src/dao/log_mapper.html")]
    pub async fn select_count(
        rb: &mut dyn Executor,
        log: &LogPageDTO,
        extend: &ExtendPageDTO,
    ) -> Result<Option<u64>, Error> {
        impled!()
    }

    /// 分页查询日志
    #[html_sql("./src/dao/log_mapper.html")]
    pub async fn select_recently(
        rb: &mut dyn Executor,
        user: &String,
    ) -> Result<Option<LogVO>, Error> {
        impled!()
    }
}
