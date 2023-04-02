use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Base64PictureDTO {
    pub name:Option<String>,
    pub content:Option<String>
}