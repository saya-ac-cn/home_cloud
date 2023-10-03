use crate::entity::table::Plan;
use serde::{Deserialize, Serialize};

/// 提醒事项展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanVO {
    pub id: Option<u64>,
    pub standard_time: Option<String>,
    pub cycle: Option<u32>,
    pub unit: Option<u32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub next_exec_time: Option<String>,
    pub organize: Option<u64>,
    pub user: Option<String>,
    pub display: Option<u32>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

impl From<Plan> for PlanVO {
    fn from(arg: Plan) -> Self {
        Self {
            id: arg.id,
            standard_time: arg.standard_time,
            cycle: arg.cycle,
            unit: arg.unit,
            title: arg.title,
            content: arg.content,
            next_exec_time: arg.next_exec_time,
            organize: arg.organize,
            user: arg.user,
            display: arg.display,
            create_time: arg.create_time,
            update_time: arg.update_time,
        }
    }
}
