use serde::{Deserialize, Serialize};
use crate::entity::domain::primary_database_tables::Plan;
use crate::util;
use crate::util::date_time::DateTimeUtil;

/// 提醒事项展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlanVO{
    pub id:Option<u64>,
    pub standard_time:Option<String>,
    pub cycle:Option<u32>,
    pub unit:Option<u32>,
    pub content:Option<String>,
    pub next_exec_time:Option<String>,
    pub organize:Option<u64>,
    pub user:Option<String>,
    pub display:Option<u32>,
    pub create_time:Option<String>,
    pub update_time:Option<String>
}

impl From<Plan> for PlanVO {
    fn from(arg: Plan) -> Self {
        Self {
            id: arg.id,
            standard_time: DateTimeUtil::naive_date_time_to_str(&arg.standard_time,&util::FORMAT_Y_M_D_H_M_S),
            cycle: arg.cycle,
            unit:arg.unit,
            content: arg.content,
            next_exec_time: DateTimeUtil::naive_date_time_to_str(&arg.next_exec_time,&util::FORMAT_Y_M_D_H_M_S),
            organize: arg.organize,
            user: arg.user,
            display: arg.display,
            create_time: DateTimeUtil::naive_date_time_to_str(&arg.create_time,&util::FORMAT_Y_M_D_H_M_S),
            update_time: DateTimeUtil::naive_date_time_to_str(&arg.update_time,&util::FORMAT_Y_M_D_H_M_S)
        }
    }
}