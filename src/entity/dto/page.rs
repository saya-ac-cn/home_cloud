use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct ExtendPageDTO{
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<chrono::NaiveDateTime>,
    pub end_time: Option<chrono::NaiveDateTime>,
}