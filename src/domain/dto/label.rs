use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LabelDTO{
    pub id:Option<u64>,
    pub name:Option<String>
}