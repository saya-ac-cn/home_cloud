use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
/// 流水明细数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneralJournalDTO {
    pub id:Option<u64>,
    pub journal_id:Option<u64>,
    pub flag:Option<String>,
    pub amount:Option<Decimal>,
    pub remarks:Option<String>,
}