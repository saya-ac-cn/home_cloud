use serde::{Deserialize, Serialize};
use crate::domain::table::DbDumpLog;

/// 数据库备份日志展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DbDumpLogVO{
    pub id:Option<u64>,
    pub url:Option<String>,
    pub archive_date:Option<String>,
    pub execute_data:Option<String>
}

impl From<DbDumpLog> for DbDumpLogVO {
    fn from(arg: DbDumpLog) -> Self {
        Self {
            id: arg.id,
            url: arg.url,
            archive_date: arg.archive_date,
            execute_data: arg.execute_data
        }
    }
}
