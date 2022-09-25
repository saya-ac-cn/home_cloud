use serde::{Deserialize, Serialize};
use crate::entity::domain::primary_database_tables::DbDumpLog;
use crate::util;
use crate::util::date_time::DateTimeUtil;

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
            archive_date: DateTimeUtil::naive_date_time_to_str(&arg.archive_date,&util::FORMAT_Y_M_D),
            execute_data: DateTimeUtil::naive_date_time_to_str(&arg.execute_data,&util::FORMAT_Y_M_D_H_M_S)
        }
    }
}