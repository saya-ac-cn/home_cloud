use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Clone, Debug)]
pub struct ExtendPageDTO{
    pub page_no: Option<u64>,
    pub page_size: Option<u64>,
    pub begin_time: Option<rbatis::DateTimeNative>,
    pub end_time: Option<rbatis::DateTimeNative>,
}