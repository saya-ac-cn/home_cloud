use serde::{Deserialize, Serialize};

/// 万能的map结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyValueVO {
    pub key: Option<u64>,
    pub val: Option<String>,
}