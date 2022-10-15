use rbatis::{Error};
use rbatis::executor::Executor;
use crate::entity::domain::primary_database_tables::LogType;
use crate::entity::vo::log_type::LogTypeVO;

crud!(LogType {});
pub struct LogTypeMapper {}

impl LogTypeMapper {
    /// 查询所有的日志类别
    #[html_sql("./src/dao/log_type_mapper.html")]
    pub async fn select_all(rb: &mut dyn Executor) -> Result<Option<Vec<LogTypeVO>>, Error> { impled!() }
}