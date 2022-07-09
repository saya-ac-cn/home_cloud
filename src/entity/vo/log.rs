use serde::{Deserialize, Serialize};
use crate::entity::domain::Log;
use crate::entity::dto::log::LogDTO;
use crate::util::date_time::DateTimeUtil;

/// 日志展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogVO{
    pub id:Option<u64>,
    pub user:Option<String>,
    pub category:Option<String>,
    pub ip:Option<String>,
    pub city:Option<String>,
    pub date:Option<String>,
}
impl_field_name_method!(LogVO{id,user,category,ip,city,date});

impl From<Log> for LogVO {
    fn from(arg:Log)->Self{
        Self{
            id:arg.id,
            user:arg.user,
            category:arg.category,
            ip:arg.ip,
            city:arg.city,
            date:DateTimeUtil::naive_date_time_to_str(&arg.date),
        }
    }
}

impl From<LogDTO> for LogVO {
    fn from(arg:LogDTO)->Self{
        Self{
            id:arg.id,
            user:arg.user,
            category:arg.category,
            ip:arg.ip,
            city:arg.city,
            date:arg.date,
        }
    }
}