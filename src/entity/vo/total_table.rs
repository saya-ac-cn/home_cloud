use serde::{Deserialize, Serialize};

/// 统计各表的数据量
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TotalTable {
    pub name: Option<String>,
    pub value: Option<i32>,
}
