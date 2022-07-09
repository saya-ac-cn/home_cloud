use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::executor::{RbatisRef, RBatisTxExecutor, ExecutorMut, RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{DateTimeNative, Error};
use crate::entity::domain::Log;
use crate::entity::dto::{ExtendPageDTO};
use crate::entity::dto::log::LogPageDTO;
use crate::entity::vo::log::LogVO;

pub struct LogMapper{}

impl LogMapper {

    /// 记录日志
    pub async fn record_log(rbatis: &Rbatis, category: String) -> Result<rbatis::Result<DBExecResult>, Error> {
        let log = Log{
            id:None,
            user:Some(String::from("Pandora")),
            category:Some(category),
            ip:Some(String::from("127.0.0.1")),
            city:Some(String::from("四川自贡")),
            date:Some(DateTimeNative::now()),
        };
        return Ok(rbatis.save(&log, &[]).await);
    }

    #[html_sql("./src/dao/log_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,log:&LogPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<LogVO>>,Error> { impled!() }

    #[html_sql("./src/dao/log_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,log:&LogPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}