use serde::{Deserialize, Serialize};

/// 日志展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogVO{
    pub id:Option<u64>,
    pub organize: Option<u64>,
    pub user:Option<String>,
    pub category:Option<String>,
    pub ip:Option<String>,
    pub city:Option<String>,
    pub date:Option<String>,
    pub detail:Option<String>
}
