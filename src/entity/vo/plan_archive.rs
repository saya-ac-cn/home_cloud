use serde::{Deserialize, Serialize};
use crate::entity::domain::primary_database_tables::PlanArchive;
use crate::util;
use crate::util::date_time::DateTimeUtil;

/// 任务归档展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanArchiveVO{
    pub id:Option<u64>,
    pub plan_id:Option<u64>,
    pub status:Option<u32>,
    pub content:Option<String>,
    pub archive_time:Option<String>,
    pub create_time:Option<String>,
    pub update_time:Option<String>
}


impl From<PlanArchive> for PlanArchiveVO {
    fn from(arg: PlanArchive) -> Self {
        Self {
            id: arg.id,
            plan_id: arg.plan_id,
            status:arg.status,
            content: arg.content,
            archive_time: DateTimeUtil::naive_date_time_to_str(&arg.archive_time,&util::FORMAT_Y_M_D_H_M_S),
            create_time: DateTimeUtil::naive_date_time_to_str(&arg.create_time,&util::FORMAT_Y_M_D_H_M_S),
            update_time: DateTimeUtil::naive_date_time_to_str(&arg.update_time,&util::FORMAT_Y_M_D_H_M_S)
        }
    }
}