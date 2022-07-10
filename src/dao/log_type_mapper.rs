use rbatis::executor::{RbatisExecutor};
use rbatis::{Error};
use crate::entity::vo::log_type::LogTypeVO;

pub struct LogTypeMapper {}

impl LogTypeMapper {
    /// 查询所有的日志类别
    #[html_sql("./src/dao/log_type_mapper.html")]
    pub async fn select_all(rb: &mut RbatisExecutor<'_, '_>) -> Result<Option<Vec<LogTypeVO>>, Error> { impled!() }
}