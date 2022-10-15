use serde::{Deserialize, Serialize};
use crate::entity::domain::primary_database_tables::PlanArchive;

/// 任务归档展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanArchiveVO{
    pub id:Option<u64>,
    pub plan_id:Option<u64>,
    pub status:Option<u32>,
    pub content:Option<String>,
    pub user_account:Option<String>,
    pub user_name:Option<String>,
    pub user_mail:Option<String>,
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
            user_account:None,
            user_name:None,
            user_mail:None,
            archive_time: arg.archive_time,
            create_time: arg.create_time,
            update_time: arg.update_time
        }
    }
}
