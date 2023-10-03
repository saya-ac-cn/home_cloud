use crate::entity::dto::db_dump_log::DbDumpLogPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::db_dump_log::DbDumpLogVO;
use rbatis::executor::Executor;
use rbatis::Error;

use crate::entity::table::DbDumpLog;
crud!(DbDumpLog {});

pub struct DbDumpLogMapper {}

impl DbDumpLogMapper {
    /// 分页数据库备份日志
    #[html_sql("./src/dao/db_dump_log_mapper.html")]
    pub async fn select_page(
        rb: &mut dyn Executor,
        log: &DbDumpLogPageDTO,
        extend: &ExtendPageDTO,
    ) -> Result<Option<Vec<DbDumpLogVO>>, Error> {
        impled!()
    }

    /// 查询数据库备份日志总数
    #[html_sql("./src/dao/db_dump_log_mapper.html")]
    pub async fn select_count(
        rb: &mut dyn Executor,
        log: &DbDumpLogPageDTO,
        extend: &ExtendPageDTO,
    ) -> Result<Option<u64>, Error> {
        impled!()
    }
}
