use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
/// 流水明细数据传输层
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GeneralJournalDTO {
    /// 主键id
    pub id:Option<u64>,
    /// 父流水id
    pub journal_id:Option<u64>,
    /// 收支方向
    pub flag:Option<String>,
    /// 金额
    pub amount:Option<Decimal>,
    /// 备注
    pub remarks:Option<String>,
    /// 会话token
    pub token: Option<String>,
}