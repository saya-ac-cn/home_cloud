use serde::{Deserialize, Serialize};
use crate::entity::domain::financial_database_tables::PaymentMeans;

/// 收支方式展示层
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaymentMeansVO{
    pub id:Option<u64>,
    pub name:Option<String>
}

impl From<PaymentMeans> for PaymentMeansVO {
    fn from(arg: PaymentMeans) -> Self {
        Self {
            id: arg.id,
            name: arg.name
        }
    }
}