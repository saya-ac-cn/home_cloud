use rbatis::executor::{RbatisExecutor};
use rbatis::db::DBExecResult;
use rbatis::{ Error};
use crate::entity::domain::primary_database_tables::DbDumpLog;
use crate::entity::dto::db_dump_log::DbDumpLogPageDTO;
use crate::entity::dto::page::ExtendPageDTO;
use crate::entity::vo::db_dump_log::DbDumpLogVO;

pub struct DbDumpLogMapper{}

impl DbDumpLogMapper {
    /// 分页数据库备份日志
    #[html_sql("./src/dao/db_dump_log_mapper.html")]
    pub async fn select_page(rb: &mut RbatisExecutor<'_,'_>,log:&DbDumpLogPageDTO,extend:&ExtendPageDTO) -> Result<Option<Vec<DbDumpLogVO>>,Error> { impled!() }

    /// 查询数据库备份日志总数
    #[html_sql("./src/dao/db_dump_log_mapper.html")]
    pub async fn select_count(rb: &mut RbatisExecutor<'_,'_>,log:&DbDumpLogPageDTO,extend:&ExtendPageDTO) -> Result<Option<u64>,Error> { impled!() }
}