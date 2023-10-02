use crate::entity::table::Monetary;
use serde::{Deserialize, Serialize};

/// 货币展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MonetaryVO {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub abbreviate: Option<String>,
    pub symbol: Option<String>,
}

impl From<Monetary> for MonetaryVO {
    fn from(arg: Monetary) -> Self {
        Self {
            id: arg.id,
            name: arg.name,
            abbreviate: arg.abbreviate,
            symbol: arg.symbol,
        }
    }
}
