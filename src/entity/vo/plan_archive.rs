use crate::entity::table::PlanArchive;
use serde::{Deserialize, Serialize};

/// 任务归档展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanArchiveVO {
    pub id: Option<u64>,
    pub status: Option<u32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub notice_user: Option<String>,
    pub open_id: Option<String>,
    pub user_name: Option<String>,
    pub user_mail: Option<String>,
    pub archive_time: Option<String>,
    pub organize: Option<u64>,
    pub user: Option<String>,
    pub display: Option<u32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

impl From<PlanArchive> for PlanArchiveVO {
    fn from(arg: PlanArchive) -> Self {
        Self {
            id: arg.id,
            status: arg.status,
            title: arg.title,
            content: arg.content,
            notice_user:arg.notice_user,
            open_id: None,
            user_name: None,
            user_mail: None,
            archive_time: arg.archive_time,
            organize: arg.organize,
            user: arg.user,
            display: arg.display,
            create_time: arg.create_time,
            update_time: arg.update_time,
        }
    }
}
